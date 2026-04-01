use crate::core::py_escape::escape_py_key;
use crate::core::transpiler::builder::PythonBuilder;
use crate::core::types::{HoudiniNode, ParamValue};
use std::collections::HashMap;

pub fn write_parameter_pass(
    builder: &mut PythonBuilder,
    nodes: &[Box<dyn HoudiniNode>],
    id_to_var: &HashMap<usize, String>,
) -> Result<(), String> {
    builder.empty_line();
    builder.line("# --- Parameter Pass ---");

    for node in nodes {
        write_node_parameters(builder, node.as_ref(), id_to_var)?;
    }
    Ok(())
}

fn write_node_parameters(
    builder: &mut PythonBuilder,
    node: &dyn HoudiniNode,
    id_to_var: &HashMap<usize, String>,
) -> Result<(), String> {
    let var_name = id_to_var
        .get(&node.get_id())
        .ok_or_else(|| format!("missing variable mapping for node id {}", node.get_id()))?;

    let mut params: Vec<_> = node.get_params().iter().collect();
    params.sort_by_key(|(k, _)| *k);

    for (key, val) in params {
        write_single_parameter(builder, var_name, key, val);
    }
    Ok(())
}

fn write_single_parameter(
    builder: &mut PythonBuilder,
    var_name: &str,
    key: &str,
    val: &ParamValue,
) {
    let safe_key = escape_py_key(key);

    if val.is_trigger() {
        builder.line(&format!("{}.parm('{}').pressButton()", var_name, safe_key));
    } else if val.is_expression() {
        let py_val = val.to_python_expr();
        builder.line(&format!(
            "{}.parm('{}').setExpression({})",
            var_name, safe_key, py_val
        ));
    } else if val.is_tuple() {
        let py_val = val.to_python_expr();
        builder.line(&format!(
            "{}.parmTuple('{}').set({})",
            var_name, safe_key, py_val
        ));
    } else {
        let py_val = val.to_python_expr();
        builder.line(&format!(
            "{}.parm('{}').set({})",
            var_name, safe_key, py_val
        ));
    }
}
