use crate::core::py_escape::escape_py_key;
use crate::core::transpiler::builder::PythonBuilder;
use crate::core::types::HoudiniNode;
use std::collections::HashMap;

pub fn write_creation_pass(
    builder: &mut PythonBuilder,
    nodes: &[Box<dyn HoudiniNode>],
    id_to_var: &HashMap<usize, String>,
) -> Result<(), String> {
    builder.line("# --- Node Creation Pass ---");
    for node in nodes {
        let var_name = id_to_var
            .get(&node.get_id())
            .ok_or_else(|| format!("missing variable mapping for node id {}", node.get_id()))?;

        let escaped_name = escape_py_key(node.get_name());
        let n_type = node.get_node_type();
        builder.line(&format!(
            "{} = parent.createNode('{}', '{}')",
            var_name, n_type, escaped_name
        ));
    }
    Ok(())
}
