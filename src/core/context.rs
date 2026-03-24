use crate::core::py_escape::{escape_py_key, sanitize_py_ident};
use crate::core::types::HoudiniNode;
use std::collections::HashMap;
use std::fmt::Write;

pub struct Transpiler {
    parent_path: String,
    nodes: Vec<Box<dyn HoudiniNode>>,
    id_to_var: HashMap<usize, String>,
}

impl Transpiler {
    pub fn new(parent_path: &str) -> Self {
        Self {
            parent_path: parent_path.to_string(),
            nodes: Vec::new(),
            id_to_var: HashMap::new(),
        }
    }

    pub fn add<T: HoudiniNode + 'static>(&mut self, node: T) {
        self.register_node(&node);
        self.nodes.push(Box::new(node));
    }

    pub fn generate_script(&self) -> String {
        let mut code = String::new();
        self.write_header(&mut code);
        self.write_creation_pass(&mut code);
        self.write_parameter_pass(&mut code);
        self.write_link_pass(&mut code);
        self.write_footer(&mut code);
        code
    }

    fn register_node(&mut self, node: &dyn HoudiniNode) {
        let safe_name = sanitize_py_ident(node.get_name());
        let var_name = format!("n_{}_{}", safe_name, node.get_id());
        assert!(
            self.id_to_var.insert(node.get_id(), var_name).is_none(),
            "duplicate node id {} while registering '{}'",
            node.get_id(),
            node.get_name()
        );
    }

    pub(crate) fn add_boxed(&mut self, node: Box<dyn HoudiniNode>) {
        self.register_node(node.as_ref());
        self.nodes.push(node);
    }

    fn write_header(&self, code: &mut String) {
        let safe_path = escape_py_key(&self.parent_path);
        let _ = writeln!(code, "import hou");
        let _ = writeln!(code, "parent = hou.node('{}')", safe_path);
        let _ = writeln!(code, "if not parent:");
        let _ = writeln!(
            code,
            "    raise RuntimeError(\"Parent node '{}' not found\")\n",
            safe_path
        );
    }

    fn write_creation_pass(&self, code: &mut String) {
        let _ = writeln!(code, "# --- 1. Node Creation Pass ---");
        for node in &self.nodes {
            let var_name = self.id_to_var.get(&node.get_id()).unwrap();
            let escaped_name = escape_py_key(node.get_name());
            let n_type = node.get_node_type();
            let _ = writeln!(
                code,
                "{} = parent.createNode('{}', '{}')",
                var_name, n_type, escaped_name
            );
        }
    }

    fn write_parameter_pass(&self, code: &mut String) {
        let _ = writeln!(code, "\n# --- 2. Parameter Pass ---");
        for node in &self.nodes {
            let var_name = self.id_to_var.get(&node.get_id()).unwrap();

            let mut params: Vec<_> = node.get_params().iter().collect();
            params.sort_by_key(|(k, _)| *k);

            for (key, val) in params {
                let py_val = val.to_python_expr();
                let safe_key = escape_py_key(key);
                if val.is_trigger() {
                    let _ = writeln!(code, "{}.parm('{}').pressButton()", var_name, safe_key);
                } else if val.is_tuple() {
                    let _ = writeln!(
                        code,
                        "{}.parmTuple('{}').set({})",
                        var_name, safe_key, py_val
                    );
                } else {
                    let _ = writeln!(code, "{}.parm('{}').set({})", var_name, safe_key, py_val);
                }
            }
        }
    }

    fn write_link_pass(&self, code: &mut String) {
        let _ = writeln!(code, "\n# --- 3. Link Pass ---");
        for node in &self.nodes {
            let var_name = self.id_to_var.get(&node.get_id()).unwrap();

            for (idx, (target_id, target_out_idx)) in node.get_inputs() {
                if let Some(target_var) = self.id_to_var.get(target_id) {
                    let _ = writeln!(
                        code,
                        "{}.setInput({}, {}, {})",
                        var_name, idx, target_var, target_out_idx
                    );
                } else {
                    eprintln!(
                        "WARNING: Target node with ID {} not found in the graph. Skipping connection for input {} of {}.",
                        target_id, idx, var_name
                    );
                    let _ = writeln!(
                        code,
                        "# WARNING: Target node ID {} not found. Connection to input {} skipped.",
                        target_id, idx
                    );
                }
            }
        }
    }

    fn write_footer(&self, code: &mut String) {
        let _ = writeln!(code, "\n# --- 4. Layout Pass ---");
        let _ = writeln!(code, "parent.layoutChildren()");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::ParamValue;
    use std::collections::{BTreeMap, HashMap};

    struct DummyNode {
        id: usize,
        name: String,
        node_type: &'static str,
        inputs: BTreeMap<usize, (usize, usize)>,
        params: HashMap<String, ParamValue>,
    }

    impl HoudiniNode for DummyNode {
        fn get_id(&self) -> usize {
            self.id
        }
        fn get_name(&self) -> &str {
            &self.name
        }
        fn get_node_type(&self) -> &'static str {
            self.node_type
        }
        fn get_inputs(&self) -> &BTreeMap<usize, (usize, usize)> {
            &self.inputs
        }
        fn get_params(&self) -> &HashMap<String, ParamValue> {
            &self.params
        }
    }

    #[test]
    fn test_transpiler_script_generation() {
        let mut node1 = DummyNode {
            id: 101,
            name: "my'box.1".to_string(),
            node_type: "box",
            inputs: BTreeMap::new(),
            params: HashMap::new(),
        };
        node1
            .params
            .insert("sizex".to_string(), ParamValue::Float(2.5));
        node1
            .params
            .insert("execute".to_string(), ParamValue::Button);

        let mut node2 = DummyNode {
            id: 102,
            name: "my color".to_string(),
            node_type: "color",
            inputs: BTreeMap::new(),
            params: HashMap::new(),
        };
        node2
            .params
            .insert("color".to_string(), ParamValue::Float3([1.0, 0.5, 0.0]));
        node2.inputs.insert(0, (101, 0));

        let mut transpiler = Transpiler::new("/obj/node's_geo");
        transpiler.add_boxed(Box::new(node1));
        transpiler.add_boxed(Box::new(node2));

        let script = transpiler.generate_script();

        assert!(script.contains("parent = hou.node('/obj/node\\'s_geo')"));

        assert!(script.contains("n_my_box_1_101 = parent.createNode('box', 'my\\'box.1')"));
        assert!(script.contains("n_my_color_102 = parent.createNode('color', 'my color')"));

        assert!(script.contains("n_my_box_1_101.parm('sizex').set(2.5000)"));
        assert!(script.contains("n_my_box_1_101.parm('execute').pressButton()"));
        assert!(script.contains("n_my_color_102.parmTuple('color').set((1.0000, 0.5000, 0.0000))"));

        assert!(script.contains("n_my_color_102.setInput(0, n_my_box_1_101, 0)"));
        assert!(script.contains("parent.layoutChildren()"));
    }

    #[test]
    fn test_missing_node_connection_warning() {
        let mut node = DummyNode {
            id: 201,
            name: "target_missing".to_string(),
            node_type: "null",
            inputs: BTreeMap::new(),
            params: HashMap::new(),
        };
        node.inputs.insert(0, (999, 0));

        let mut transpiler = Transpiler::new("/obj/geo1");
        transpiler.add_boxed(Box::new(node));

        let script = transpiler.generate_script();
        assert!(script.contains("# WARNING: Target node ID 999 not found."));
        assert!(!script.contains("n_target_missing_201.setInput(0,"));
    }

    #[test]
    fn test_same_name_nodes_get_distinct_python_vars() {
        let node1 = DummyNode {
            id: 1,
            name: "dup".to_string(),
            node_type: "box",
            inputs: BTreeMap::new(),
            params: HashMap::new(),
        };
        let mut node2 = DummyNode {
            id: 2,
            name: "dup".to_string(),
            node_type: "color",
            inputs: BTreeMap::new(),
            params: HashMap::new(),
        };
        node2.inputs.insert(0, (1, 0));

        let mut transpiler = Transpiler::new("/obj/geo1");
        transpiler.add(node1);
        transpiler.add(node2);

        let script = transpiler.generate_script();
        assert!(script.contains("n_dup_1 = parent.createNode('box', 'dup')"));
        assert!(script.contains("n_dup_2 = parent.createNode('color', 'dup')"));
        assert!(script.contains("n_dup_2.setInput(0, n_dup_1, 0)"));
    }

    #[test]
    #[should_panic(expected = "duplicate node id")]
    fn test_duplicate_node_id_is_rejected() {
        let mut transpiler = Transpiler::new("/obj/geo1");
        transpiler.add(DummyNode {
            id: 42,
            name: "a".to_string(),
            node_type: "box",
            inputs: BTreeMap::new(),
            params: HashMap::new(),
        });
        transpiler.add(DummyNode {
            id: 42,
            name: "b".to_string(),
            node_type: "color",
            inputs: BTreeMap::new(),
            params: HashMap::new(),
        });
    }
}
