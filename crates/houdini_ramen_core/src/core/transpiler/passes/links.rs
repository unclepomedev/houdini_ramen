use crate::core::py_escape::escape_py_key;
use crate::core::transpiler::builder::PythonBuilder;
use crate::core::types::{HoudiniNode, OutputPin};
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
    let Some(var_name) = id_to_var.get(&node.get_id()) else {
        emit_warning(
            builder,
            format!(
                "WARNING: Source node with ID {} missing variable mapping. Skipping link emission.",
                node.get_id()
            ),
            format!(
                "Source node ID {} missing variable mapping. Links skipped.",
                node.get_id()
            ),
        );
        return;
    };

    for (idx, (target_id, target_out_pin)) in node.get_inputs() {
        write_single_link(
            builder,
            var_name,
            *idx,
            *target_id,
            target_out_pin,
            id_to_var,
        );
    }
}

fn write_single_link(
    builder: &mut PythonBuilder,
    var_name: &str,
    input_idx: usize,
    target_id: usize,
    target_out_pin: &OutputPin,
    id_to_var: &HashMap<usize, String>,
) {
    if let Some(target_var) = id_to_var.get(&target_id) {
        match target_out_pin {
            OutputPin::Index(idx) => {
                builder.line(&format!(
                    "{}.setInput({}, {}, {})",
                    var_name, input_idx, target_var, idx
                ));
            }
            OutputPin::Name(name) => {
                let safe_name = escape_py_key(name);
                builder.line(&format!(
                    "try:\n    {}.setInput({}, {}, {}.outputIndex('{}'))\nexcept hou.OperationFailed:\n    print(f\"Warning: Could not resolve output pin '{}' on {{{}.path()}}\")",
                    var_name, input_idx, target_var, target_var, safe_name, safe_name, target_var
                ));
            }
        }
    } else {
        emit_warning(
            builder,
            format!(
                "WARNING: Target node with ID {} not found in the graph. Skipping connection for input {} of {}.",
                target_id, input_idx, var_name
            ),
            format!(
                "Target node ID {} not found. Connection to input {} skipped.",
                target_id, input_idx
            ),
        );
    }
}

fn emit_warning(builder: &mut PythonBuilder, stderr_msg: String, py_comment: String) {
    eprintln!("{stderr_msg}");
    builder.line(&format!("# WARNING: {py_comment}"));
}
