use super::Transpiler;
use crate::core::types::{ContainerType, HoudiniNode, ParamValue, SpareParam};
use std::collections::{BTreeMap, HashMap};

struct DummyNode {
    id: usize,
    name: String,
    node_type: &'static str,
    inputs: BTreeMap<usize, (usize, usize)>,
    params: HashMap<String, ParamValue>,
    spare_params: Vec<SpareParam>,
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
    fn get_spare_params(&self) -> &Vec<SpareParam> {
        &self.spare_params
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
        spare_params: vec![],
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
        spare_params: vec![],
    };
    node2
        .params
        .insert("color".to_string(), ParamValue::Float3([1.0, 0.5, 0.0]));
    node2.inputs.insert(0, (101, 0));

    let mut transpiler = Transpiler::new("/obj/node's_geo", None, false);
    transpiler.add_boxed(Box::new(node1));
    transpiler.add_boxed(Box::new(node2));

    let script = transpiler.generate_script();

    assert!(script.contains("parent_path = '/obj/node\\'s_geo'"));
    assert!(script.contains("parent = hou.node(parent_path)"));

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
        spare_params: vec![],
    };
    node.inputs.insert(0, (999, 0));

    let mut transpiler = Transpiler::new("/obj/geo1", None, false);
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
        spare_params: vec![],
    };
    let mut node2 = DummyNode {
        id: 2,
        name: "dup".to_string(),
        node_type: "color",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![],
    };
    node2.inputs.insert(0, (1, 0));

    let mut transpiler = Transpiler::new("/obj/geo1", None, false);
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
    let mut transpiler = Transpiler::new("/obj/geo1", None, false);
    transpiler.add(DummyNode {
        id: 42,
        name: "a".to_string(),
        node_type: "box",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![],
    });
    transpiler.add(DummyNode {
        id: 42,
        name: "b".to_string(),
        node_type: "color",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![],
    });
}

#[test]
fn test_transpiler_auto_create() {
    let transpiler = Transpiler::new("/obj/my_auto_geo", Some(ContainerType::Geo), false);
    let script = transpiler.generate_script();

    assert!(script.contains("if not parent:"));
    assert!(script.contains("parts = [p for p in parent_path.split('/') if p]"));
    assert!(script.contains("n_type = 'geo' if i == len(parts) - 1 else 'subnet'"));
    assert!(script.contains("curr = curr.createNode(n_type, part)"));
}

#[test]
fn test_transpiler_auto_clear() {
    let transpiler = Transpiler::new("/obj/geo1", None, true);
    let script = transpiler.generate_script();

    assert!(script.contains("for child in parent.children():"));
    assert!(script.contains("child.destroy()"));
}

#[test]
fn test_transpiler_auto_create_and_clear() {
    let transpiler = Transpiler::new("/obj/geo_test", Some(ContainerType::Geo), true);
    let script = transpiler.generate_script();

    let create_idx = script.find("curr = curr.createNode(n_type, part)").unwrap();
    let clear_idx = script.find("for child in parent.children():").unwrap();

    assert!(
        create_idx < clear_idx,
        "Auto-create should happen before auto-clear"
    );
}

#[test]
fn test_transpiler_spare_parameters() {
    let node = DummyNode {
        id: 301,
        name: "ctrl".to_string(),
        node_type: "null",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![SpareParam::Float {
            name: "radius".to_string(),
            label: "Radius".to_string(),
            default: 2.5,
            min: 0.1,
            max: 10.0,
        }],
    };

    let mut transpiler = Transpiler::new("/obj/geo1", None, false);
    transpiler.add_boxed(Box::new(node));
    let script = transpiler.generate_script();

    assert!(script.contains("ptg = n_ctrl_301.parmTemplateGroup()"));
    assert!(script.contains(
        "pt = hou.FloatParmTemplate('radius', 'Radius', 1, default_value=(2.5,), min=0.1, max=10.0)"
    ));
    assert!(script.contains("ptg.append(pt)"));
    assert!(script.contains("n_ctrl_301.setParmTemplateGroup(ptg)"));
}
