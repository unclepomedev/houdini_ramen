use crate::core::transpiler::builder::PythonBuilder;
use crate::core::types::InputPort;
use crate::core::types::{HoudiniNode, OutputPort};
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

    for (in_port, (target_id, target_out_port)) in node.get_inputs() {
        write_single_link(
            builder,
            var_name,
            in_port,
            *target_id,
            target_out_port,
            id_to_var,
        );
    }
}

fn write_single_link(
    builder: &mut PythonBuilder,
    var_name: &str,
    input_port: &InputPort,
    target_id: usize,
    target_out_port: &OutputPort,
    id_to_var: &HashMap<usize, String>,
) {
    let in_arg_log = match input_port {
        InputPort::Index(i) => i.to_string(),
        InputPort::Name(name) => name.clone(),
    };

    if let Some(target_var) = id_to_var.get(&target_id) {
        let in_arg = match input_port {
            InputPort::Index(i) => i.to_string(),
            InputPort::Name(name) => format!("'{}'", name),
        };

        let out_arg = match target_out_port {
            OutputPort::Index(i) => i.to_string(),
            OutputPort::Name(name) => format!("{}.outputNames().index('{}')", target_var, name),
        };

        builder.line(&format!(
            "{}.setInput({}, {}, {})",
            var_name, in_arg, target_var, out_arg
        ));
    } else {
        emit_warning(
            builder,
            format!(
                "WARNING: Target node with ID {} not found in the graph. Skipping connection for input {} of {}.",
                target_id, in_arg_log, var_name
            ),
            format!(
                "Target node ID {} not found. Connection to input {} skipped.",
                target_id, in_arg_log
            ),
        );
    }
}

fn emit_warning(builder: &mut PythonBuilder, stderr_msg: String, py_comment: String) {
    eprintln!("{stderr_msg}");
    builder.line(&format!("# WARNING: {py_comment}"));
}
