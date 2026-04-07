use super::Transpiler;
use crate::core::graph::{ExistingNodeRef, NodeGraph};
use crate::core::types::{ContainerType, HoudiniNode, ParamValue, SpareParam};
use std::collections::{BTreeMap, HashMap};

#[derive(Clone)]
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
    fn get_spare_params(&self) -> &[SpareParam] {
        &self.spare_params
    }
}

#[derive(Clone)]
struct DummyContainerNode {
    id: usize,
    name: String,
    node_type: &'static str,
    dive_target: &'static str,
    inputs: BTreeMap<usize, (usize, usize)>,
    params: HashMap<String, ParamValue>,
    spare_params: Vec<SpareParam>,
}

impl HoudiniNode for DummyContainerNode {
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
    fn get_spare_params(&self) -> &[SpareParam] {
        &self.spare_params
    }
    fn get_dive_target(&self) -> Option<&'static str> {
        Some(self.dive_target)
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
    transpiler.add_boxed(Box::new(node1)).unwrap();
    transpiler.add_boxed(Box::new(node2)).unwrap();

    let script = transpiler.generate_script().unwrap();

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
    transpiler.add_boxed(Box::new(node)).unwrap();

    let script = transpiler.generate_script().unwrap();
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
    transpiler.add(node1).unwrap();
    transpiler.add(node2).unwrap();

    let script = transpiler.generate_script().unwrap();
    assert!(script.contains("n_dup_1 = parent.createNode('box', 'dup')"));
    assert!(script.contains("n_dup_2 = parent.createNode('color', 'dup')"));
    assert!(script.contains("n_dup_2.setInput(0, n_dup_1, 0)"));
}

#[test]
fn test_transpiler_extended_spare_parameters() {
    let node = DummyNode {
        id: 302,
        name: "extended_ctrl".to_string(),
        node_type: "null",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![
            SpareParam::Color {
                name: "bg_color".to_string(),
                label: "Background Color".to_string(),
                default: [0.1, 0.5, 0.9],
            },
            SpareParam::Button {
                name: "execute_script".to_string(),
                label: "Execute".to_string(),
            },
            SpareParam::Menu {
                name: "shape".to_string(),
                label: "Shape Type".to_string(),
                default: 1,
                menu_items: vec![
                    ("box".to_string(), "Box".to_string()),
                    ("sphere".to_string(), "Sphere".to_string()),
                ],
            },
            SpareParam::File {
                name: "texture_path".to_string(),
                label: "Texture".to_string(),
                default: "$HIP/tex.rat".to_string(),
            },
            SpareParam::NodePath {
                name: "target_geo".to_string(),
                label: "Target".to_string(),
                default: "../geo1".to_string(),
            },
            SpareParam::RampFloat {
                name: "falloff".to_string(),
                label: "Falloff Curve".to_string(),
            },
            SpareParam::RampColor {
                name: "color_remap".to_string(),
                label: "Color Remap".to_string(),
            },
        ],
    };

    let mut transpiler = Transpiler::new("/obj/geo1", None, false);
    transpiler.add_boxed(Box::new(node)).unwrap();
    let script = transpiler.generate_script().unwrap();

    assert!(script.contains("ptg = n_extended_ctrl_302.parmTemplateGroup()"));

    assert!(script.contains(
        r#"pt = hou.FloatParmTemplate("bg_color", "Background Color", 3, default_value=(0.1000, 0.5000, 0.9000), look=hou.parmLook.ColorSquare)"#
    ));
    assert!(script.contains(r#"pt = hou.ButtonParmTemplate("execute_script", "Execute")"#));
    assert!(script.contains(
        r#"pt = hou.MenuParmTemplate("shape", "Shape Type", menu_items=("box", "sphere",), menu_labels=("Box", "Sphere",), default_value=1)"#
    ));
    assert!(script.contains(
        r#"pt = hou.StringParmTemplate("texture_path", "Texture", 1, default_value=("$HIP/tex.rat",), string_type=hou.stringParmType.FileReference)"#
    ));
    assert!(script.contains(
        r#"pt = hou.StringParmTemplate("target_geo", "Target", 1, default_value=("../geo1",), string_type=hou.stringParmType.NodeReference)"#
    ));
    assert!(script.contains(
        r#"pt = hou.RampParmTemplate("falloff", "Falloff Curve", hou.rampParmType.Float)"#
    ));
    assert!(script.contains(
        r#"pt = hou.RampParmTemplate("color_remap", "Color Remap", hou.rampParmType.Color)"#
    ));

    assert!(script.contains("ptg.append(pt)"));
    assert!(script.contains("n_extended_ctrl_302.setParmTemplateGroup(ptg)"));
}

#[test]
fn test_duplicate_node_id_is_rejected() {
    let mut transpiler = Transpiler::new("/obj/geo1", None, false);
    transpiler
        .add(DummyNode {
            id: 42,
            name: "a".to_string(),
            node_type: "box",
            inputs: BTreeMap::new(),
            params: HashMap::new(),
            spare_params: vec![],
        })
        .unwrap();
    transpiler
        .add(DummyNode {
            id: 42,
            name: "b".to_string(),
            node_type: "color",
            inputs: BTreeMap::new(),
            params: HashMap::new(),
            spare_params: vec![],
        })
        .unwrap_err();
}

#[test]
fn test_transpiler_auto_create() {
    let transpiler = Transpiler::new("/obj/my_auto_geo", Some(ContainerType::Geo), false);
    let script = transpiler.generate_script().unwrap();

    assert!(script.contains("if not parent:"));
    assert!(script.contains("parts = [p for p in parent_path.split('/') if p]"));
    assert!(script.contains("n_type = 'geo' if i == len(parts) - 1 else 'subnet'"));
    assert!(script.contains("curr = curr.createNode(n_type, part)"));
}

#[test]
fn test_transpiler_auto_clear() {
    let transpiler = Transpiler::new("/obj/geo1", None, true);
    let script = transpiler.generate_script().unwrap();

    assert!(script.contains("for child in parent.children():"));
    assert!(script.contains("child.destroy()"));
}

#[test]
fn test_transpiler_auto_create_and_clear() {
    let transpiler = Transpiler::new("/obj/geo_test", Some(ContainerType::Geo), true);
    let script = transpiler.generate_script().unwrap();

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
        spare_params: vec![
            SpareParam::Float {
                name: "radius".to_string(),
                label: "Radius".to_string(),
                default: 2.5,
                min: 0.1,
                max: 10.0,
            },
            SpareParam::Int {
                name: "count".to_string(),
                label: "Count".to_string(),
                default: 500,
                min: 10,
                max: 1000,
            },
            SpareParam::String {
                name: "my_str".to_string(),
                label: "My String".to_string(),
                default: "hello\nworld".to_string(),
            },
            SpareParam::Toggle {
                name: "enable".to_string(),
                label: "Enable".to_string(),
                default: true,
            },
        ],
    };

    let mut transpiler = Transpiler::new("/obj/geo1", None, false);
    transpiler.add_boxed(Box::new(node)).unwrap();
    let script = transpiler.generate_script().unwrap();

    assert!(script.contains("ptg = n_ctrl_301.parmTemplateGroup()"));
    assert!(script.contains(
        r#"pt = hou.FloatParmTemplate("radius", "Radius", 1, default_value=(2.5,), min=0.1, max=10.0)"#
    ));
    assert!(script.contains(
        r#"pt = hou.IntParmTemplate("count", "Count", 1, default_value=(500,), min=10, max=1000)"#
    ));
    assert!(script.contains(
        r#"pt = hou.StringParmTemplate("my_str", "My String", 1, default_value=("hello\nworld",))"#
    ));
    assert!(
        script.contains(r#"pt = hou.ToggleParmTemplate("enable", "Enable", default_value=True)"#)
    );

    assert!(script.contains("ptg.append(pt)"));
    assert!(script.contains("n_ctrl_301.setParmTemplateGroup(ptg)"));
}

#[test]
fn test_transpiler_display_flag() {
    let node1 = DummyNode {
        id: 401,
        name: "process".to_string(),
        node_type: "box",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![],
    };

    let node2 = DummyNode {
        id: 402,
        name: "out_geo".to_string(),
        node_type: "null",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![],
    };

    let mut transpiler = Transpiler::new("/obj/geo1", None, false);
    transpiler.add_boxed(Box::new(node1)).unwrap();
    transpiler.add_boxed(Box::new(node2)).unwrap();

    transpiler.set_display_node(402);

    let script = transpiler.generate_script().unwrap();

    assert!(script.contains("n_out_geo_402.setDisplayFlag(True)"));
    assert!(script.contains("n_out_geo_402.setRenderFlag(True)"));

    assert!(!script.contains("n_process_401.setDisplayFlag(True)"));
    assert!(!script.contains("n_process_401.setRenderFlag(True)"));
}

#[test]
fn test_transpiler_nested_subnet_creation() {
    // Simulate: solver node at top level, with inner nodes created inside it
    let solver = DummyNode {
        id: 501,
        name: "solver".to_string(),
        node_type: "solver",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![],
    };

    let mut transpiler = Transpiler::new("/obj/geo1", None, false);
    transpiler.add_boxed(Box::new(solver)).unwrap();

    // Add an existing node (pre-created by Houdini inside the solver)
    let existing = ExistingNodeRef {
        id: 502,
        name: "Prev_Frame".to_string(),
        inputs: BTreeMap::new(),
    };
    transpiler
        .add_existing_node(Box::new(existing), 501, "Prev_Frame")
        .unwrap();

    // Add a nested node created inside the solver
    let inner_ray = DummyNode {
        id: 503,
        name: "inner_ray".to_string(),
        node_type: "ray",
        inputs: {
            let mut m = BTreeMap::new();
            m.insert(0, (502, 0));
            m
        },
        params: HashMap::new(),
        spare_params: vec![],
    };
    transpiler
        .add_boxed_with_parent(Box::new(inner_ray), 501)
        .unwrap();

    let script = transpiler.generate_script().unwrap();

    // solver created under parent
    assert!(script.contains("n_solver_501 = parent.createNode('solver', 'solver')"));

    // Prev_Frame fetched via hou.node
    assert!(script.contains("n_Prev_Frame_502 = hou.node(n_solver_501.path() + '/Prev_Frame')"));

    // inner_ray created under solver, not parent
    assert!(script.contains("n_inner_ray_503 = n_solver_501.createNode('ray', 'inner_ray')"));
    assert!(!script.contains("parent.createNode('ray', 'inner_ray')"));

    // link from inner_ray to Prev_Frame
    assert!(script.contains("n_inner_ray_503.setInput(0, n_Prev_Frame_502, 0)"));
}

#[test]
fn test_node_graph_dive_into_api() {
    let mut graph = NodeGraph::new("/obj/geo1");

    let solver = graph.add(DummyNode {
        id: 601,
        name: "solver".to_string(),
        node_type: "solver",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![],
    });

    graph.dive_into(&solver, |inner| {
        let prev_frame = inner.existing_node("Prev_Frame");
        let prev_frame_id = prev_frame.get_id();

        let _inner_node = inner.add(DummyNode {
            id: 603,
            name: "inner_ray".to_string(),
            node_type: "ray",
            inputs: {
                let mut m = BTreeMap::new();
                m.insert(0, (prev_frame_id, 0));
                m
            },
            params: HashMap::new(),
            spare_params: vec![],
        });
    });

    let script = graph.build();

    assert!(script.contains("n_solver_601 = parent.createNode('solver', 'solver')"));
    assert!(script.contains("hou.node(n_solver_601.path() + '/Prev_Frame')"));
    assert!(script.contains(".createNode('ray', 'inner_ray')"));
    // inner_ray must NOT be created under parent
    assert!(!script.contains("parent.createNode('ray', 'inner_ray')"));
    // nested container must get layoutChildren
    assert!(script.contains("n_solver_601.layoutChildren()"));
}

#[test]
fn test_nested_display_flag() {
    let mut graph = NodeGraph::new("/obj/geo1");

    let solver = graph.add(DummyNode {
        id: 701,
        name: "solver".to_string(),
        node_type: "solver",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![],
    });

    graph.dive_into(&solver, |inner| {
        let ray = inner.add(DummyNode {
            id: 702,
            name: "inner_ray".to_string(),
            node_type: "ray",
            inputs: BTreeMap::new(),
            params: HashMap::new(),
            spare_params: vec![],
        });
        inner.set_display(&ray);
    });

    let script = graph.build();

    assert!(script.contains("n_inner_ray_702.setDisplayFlag(True)"));
    assert!(script.contains("n_inner_ray_702.setRenderFlag(True)"));
}

#[test]
fn test_existing_node_runtime_guard() {
    let mut transpiler = Transpiler::new("/obj/geo1", None, false);

    let solver = DummyNode {
        id: 801,
        name: "solver".to_string(),
        node_type: "solver",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![],
    };
    transpiler.add_boxed(Box::new(solver)).unwrap();

    let existing = ExistingNodeRef {
        id: 802,
        name: "Prev_Frame".to_string(),
        inputs: BTreeMap::new(),
    };
    transpiler
        .add_existing_node(Box::new(existing), 801, "Prev_Frame")
        .unwrap();

    let script = transpiler.generate_script().unwrap();

    assert!(script.contains("if not n_Prev_Frame_802:"));
    assert!(script.contains("raise RuntimeError("));
    assert!(script.contains("Existing node not found:"));
}

#[test]
fn test_topological_sort_parent_before_child() {
    let mut transpiler = Transpiler::new("/obj/geo1", None, false);

    let outer = DummyNode {
        id: 901,
        name: "outer_subnet".to_string(),
        node_type: "subnet",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![],
    };
    transpiler.add_boxed(Box::new(outer)).unwrap();

    let inner = DummyNode {
        id: 902,
        name: "inner_subnet".to_string(),
        node_type: "subnet",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![],
    };
    transpiler
        .add_boxed_with_parent(Box::new(inner), 901)
        .unwrap();

    let existing = ExistingNodeRef {
        id: 903,
        name: "Input_1".to_string(),
        inputs: BTreeMap::new(),
    };
    transpiler
        .add_existing_node(Box::new(existing), 902, "Input_1")
        .unwrap();

    let script = transpiler.generate_script().unwrap();

    let outer_pos = script.find("n_outer_subnet_901").unwrap();
    let inner_pos = script.find("n_inner_subnet_902").unwrap();
    let existing_pos = script.find("n_Input_1_903").unwrap();

    assert!(
        outer_pos < inner_pos,
        "outer subnet must be created before inner subnet"
    );
    assert!(
        inner_pos < existing_pos,
        "inner subnet must be created before existing child"
    );
}

#[test]
fn test_dive_target_parent_resolution() {
    let mut transpiler = Transpiler::new("/obj/geo1", None, false);

    let container = DummyContainerNode {
        id: 1001,
        name: "vellum_solver".to_string(),
        node_type: "vellumsolver",
        dive_target: "inner_net",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![],
    };
    transpiler.add_boxed(Box::new(container)).unwrap();

    let child = DummyNode {
        id: 1002,
        name: "child_ray".to_string(),
        node_type: "ray",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![],
    };
    transpiler
        .add_boxed_with_parent(Box::new(child), 1001)
        .unwrap();

    let script = transpiler.generate_script().unwrap();

    assert!(
        script
            .contains("n_vellum_solver_1001 = parent.createNode('vellumsolver', 'vellum_solver')"),
        "container should be created under parent"
    );
    assert!(
        script.contains(
            "n_child_ray_1002 = n_vellum_solver_1001.node('inner_net').createNode('ray', 'child_ray')"
        ),
        "child should be created under the dive target, not directly under the container"
    );
    assert!(
        !script.contains("parent.createNode('ray', 'child_ray')"),
        "child must not be created under root parent"
    );
    // dive target container must get layoutChildren
    assert!(
        script.contains("n_vellum_solver_1001.node('inner_net').layoutChildren()"),
        "dive target container must get layoutChildren"
    );
}

#[test]
fn test_expression_parameter_emits_set_expression() {
    let mut node = DummyNode {
        id: 1101,
        name: "xform".to_string(),
        node_type: "xform",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![],
    };
    node.params.insert(
        "tx".to_string(),
        ParamValue::Expression("$F*23".to_string()),
    );
    node.params.insert(
        "ty".to_string(),
        ParamValue::Expression("ch(\"../other/ty\")".to_string()),
    );

    let mut transpiler = Transpiler::new("/obj/geo1", None, false);
    transpiler.add_boxed(Box::new(node)).unwrap();

    let script = transpiler.generate_script().unwrap();

    assert!(
        script.contains(r#"n_xform_1101.parm('tx').setExpression("$F*23")"#),
        "Expression param should emit setExpression: {}",
        script
    );
    assert!(
        script.contains(r#"n_xform_1101.parm('ty').setExpression("ch(\"../other/ty\")""#),
        "Expression with quotes should be properly escaped: {}",
        script
    );
}

#[test]
fn test_existing_node_input_wiring() {
    let mut graph = NodeGraph::new("/obj/geo1");

    let solver = graph.add(DummyNode {
        id: 1201,
        name: "solver".to_string(),
        node_type: "solver",
        inputs: BTreeMap::new(),
        params: HashMap::new(),
        spare_params: vec![],
    });

    graph.dive_into(&solver, |inner| {
        let ray1 = inner.add(DummyNode {
            id: 1202,
            name: "ray1".to_string(),
            node_type: "ray",
            inputs: BTreeMap::new(),
            params: HashMap::new(),
            spare_params: vec![],
        });
        let ray2 = inner.add(DummyNode {
            id: 1203,
            name: "ray2".to_string(),
            node_type: "ray",
            inputs: BTreeMap::new(),
            params: HashMap::new(),
            spare_params: vec![],
        });

        let out = inner.existing_node("OUT");
        inner.connect_existing(&out, 0, &ray1);

        let out_2 = inner.existing_node("OUT_2");
        inner.connect_existing_from(&out_2, 0, &ray2, 1);
    });

    let script = graph.build();

    assert!(script.contains("/OUT"));
    assert!(script.contains(".setInput(0, n_ray1_1202, 0)"));

    assert!(script.contains("/OUT_2"));
    assert!(script.contains(".setInput(0, n_ray2_1203, 1)"));
}
