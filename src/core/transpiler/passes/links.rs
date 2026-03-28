use crate::core::transpiler::builder::PythonBuilder;
use crate::core::types::HoudiniNode;
use std::collections::HashMap;

pub fn write_link_pass(
    builder: &mut PythonBuilder,
    nodes: &[Box<dyn HoudiniNode>],
    id_to_var: &HashMap<usize, String>,
) {
    builder.empty_line();
    builder.line("# --- Link Pass ---");

    for node in nodes {
        write_node_links(builder, node.as_ref(), id_to_var);
    }
}

fn write_node_links(
    builder: &mut PythonBuilder,
    node: &dyn HoudiniNode,
    id_to_var: &HashMap<usize, String>,
) {
    let var_name = id_to_var
        .get(&node.get_id())
        .unwrap_or_else(|| panic!("missing variable mapping for node id {}", node.get_id()));

    for (idx, (target_id, target_out_idx)) in node.get_inputs() {
        write_single_link(
            builder,
            var_name,
            *idx,
            *target_id,
            *target_out_idx,
            id_to_var,
        );
    }
}

fn write_single_link(
    builder: &mut PythonBuilder,
    var_name: &str,
    input_idx: usize,
    target_id: usize,
    target_out_idx: usize,
    id_to_var: &HashMap<usize, String>,
) {
    if let Some(target_var) = id_to_var.get(&target_id) {
        builder.line(&format!(
            "{}.setInput({}, {}, {})",
            var_name, input_idx, target_var, target_out_idx
        ));
    } else {
        eprintln!(
            "WARNING: Target node with ID {} not found in the graph. Skipping connection for input {} of {}.",
            target_id, input_idx, var_name
        );
        builder.line(&format!(
            "# WARNING: Target node ID {} not found. Connection to input {} skipped.",
            target_id, input_idx
        ));
    }
}
