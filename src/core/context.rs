use crate::core::py_escape::{escape_py_key, sanitize_py_ident};
use crate::core::types::HoudiniNode;
use std::fmt::Write;

pub struct Transpiler {
    parent_path: String,
    nodes: Vec<Box<dyn HoudiniNode>>,
}

impl Transpiler {
    pub fn new(parent_path: &str) -> Self {
        Self {
            parent_path: parent_path.to_string(),
            nodes: Vec::new(),
        }
    }

    pub fn add<T: HoudiniNode + 'static>(&mut self, node: T) {
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

    fn write_header(&self, code: &mut String) {
        let _ = writeln!(code, "import hou");
        let _ = writeln!(code, "parent = hou.node('{}')", self.parent_path);
        let _ = writeln!(code, "if not parent:");
        let _ = writeln!(
            code,
            "    raise RuntimeError(\"Parent node '{}' not found\")\n",
            self.parent_path
        );
    }

    fn write_creation_pass(&self, code: &mut String) {
        let _ = writeln!(code, "# --- 1. Node Creation Pass ---");
        for node in &self.nodes {
            let name = node.get_name();
            let safe_name = sanitize_py_ident(name);
            let n_type = node.get_node_type();
            let _ = writeln!(
                code,
                "n_{} = parent.createNode('{}', '{}')",
                safe_name, n_type, name
            );
        }
    }

    fn write_parameter_pass(&self, code: &mut String) {
        let _ = writeln!(code, "\n# --- 2. Parameter Pass ---");
        for node in &self.nodes {
            let safe_name = sanitize_py_ident(node.get_name());
            let var_name = format!("n_{}", safe_name);

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
            let safe_name = sanitize_py_ident(node.get_name());
            let var_name = format!("n_{}", safe_name);

            for (idx, target_name) in node.get_inputs() {
                let target_safe = sanitize_py_ident(target_name);
                let _ = writeln!(code, "{}.setInput({}, n_{}, 0)", var_name, idx, target_safe);
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
        name: String,
        node_type: &'static str,
        inputs: BTreeMap<usize, String>,
        params: HashMap<String, ParamValue>,
    }

    impl HoudiniNode for DummyNode {
        fn get_name(&self) -> &str {
            &self.name
        }
        fn get_node_type(&self) -> &'static str {
            self.node_type
        }
        fn get_inputs(&self) -> &BTreeMap<usize, String> {
            &self.inputs
        }
        fn get_params(&self) -> &HashMap<String, ParamValue> {
            &self.params
        }
    }

    #[test]
    fn test_transpiler_script_generation() {
        let mut node1 = DummyNode {
            name: "my-box.1".to_string(),
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
            name: "my color".to_string(),
            node_type: "color",
            inputs: BTreeMap::new(),
            params: HashMap::new(),
        };
        node2
            .params
            .insert("color".to_string(), ParamValue::Float3([1.0, 0.5, 0.0]));
        node2.inputs.insert(0, "my-box.1".to_string());

        let mut transpiler = Transpiler::new("/obj/geo1");
        transpiler.add(node1);
        transpiler.add(node2);

        let script = transpiler.generate_script();

        assert!(script.contains("parent = hou.node('/obj/geo1')"));

        assert!(script.contains("n_my_box_1 = parent.createNode('box', 'my-box.1')"));
        assert!(script.contains("n_my_color = parent.createNode('color', 'my color')"));

        assert!(script.contains("n_my_box_1.parm('sizex').set(2.5000)"));
        assert!(script.contains("n_my_box_1.parm('execute').pressButton()"));
        assert!(script.contains("n_my_color.parmTuple('color').set((1.0000, 0.5000, 0.0000))"));

        assert!(script.contains("n_my_color.setInput(0, n_my_box_1, 0)"));
        assert!(script.contains("parent.layoutChildren()"));
    }
}
