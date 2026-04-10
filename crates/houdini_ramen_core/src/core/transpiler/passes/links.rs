use crate::core::py_escape::{python_string_literal, sanitize_py_ident};
use crate::core::transpiler::builder::PythonBuilder;
use crate::core::types::{HoudiniNode, InputPin, OutputPin};
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

    for (in_pin, (target_id, target_out_pin)) in node.get_inputs() {
        write_single_link(
            builder,
            var_name,
            in_pin,
            *target_id,
            target_out_pin,
            id_to_var,
        );
    }
}

fn write_single_link(
    builder: &mut PythonBuilder,
    var_name: &str,
    input_pin: &InputPin,
    target_id: usize,
    target_out_pin: &OutputPin,
    id_to_var: &HashMap<usize, String>,
) {
    if let Some(target_var) = id_to_var.get(&target_id) {
        let in_idx_expr = resolve_input_pin(builder, var_name, input_pin);
        let out_idx_expr = resolve_output_pin(builder, target_var, target_out_pin);

        builder.line(&format!(
            "{}.setInput({}, {}, {})",
            var_name, in_idx_expr, target_var, out_idx_expr
        ));
    } else {
        emit_warning(
            builder,
            format!(
                "WARNING: Target node with ID {} not found in the graph. Skipping connection for input {:?} of {}.",
                target_id, input_pin, var_name
            ),
            format!(
                "Target node ID {} not found. Connection to input {:?} skipped.",
                target_id, input_pin
            ),
        );
    }
}

fn resolve_input_pin(builder: &mut PythonBuilder, var_name: &str, input_pin: &InputPin) -> String {
    match input_pin {
        InputPin::Index(idx) => idx.to_string(),
        InputPin::Name(name) => {
            let safe_name = python_string_literal(name);
            let var = format!("_in_{}_{}", var_name, sanitize_py_ident(name));

            builder.line("try:");
            builder.indent();
            builder.line(&format!("{} = {}.inputIndex({})", var, var_name, safe_name));
            builder.dedent();
            builder.line("except hou.OperationFailed:");
            builder.indent();
            builder.line(&format!(
                "raise hou.OperationFailed('Could not resolve input pin ' + repr({}) + ' on ' + {}.path())",
                safe_name, var_name
            ));
            builder.dedent();

            var
        }
    }
}

fn resolve_output_pin(
    builder: &mut PythonBuilder,
    target_var: &str,
    target_out_pin: &OutputPin,
) -> String {
    match target_out_pin {
        OutputPin::Index(idx) => idx.to_string(),
        OutputPin::Name(name) => {
            let safe_name = python_string_literal(name);
            let var = format!("_out_{}_{}", target_var, sanitize_py_ident(name));

            builder.line("try:");
            builder.indent();
            builder.line(&format!(
                "{} = {}.outputIndex({})",
                var, target_var, safe_name
            ));
            builder.dedent();
            builder.line("except hou.OperationFailed:");
            builder.indent();
            builder.line(&format!(
                "raise hou.OperationFailed('Could not resolve output pin ' + repr({}) + ' on ' + {}.path())",
                safe_name, target_var
            ));
            builder.dedent();

            var
        }
    }
}

fn emit_warning(builder: &mut PythonBuilder, stderr_msg: String, py_comment: String) {
    eprintln!("{stderr_msg}");
    builder.line(&format!("# WARNING: {py_comment}"));
}
