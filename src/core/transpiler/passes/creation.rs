use crate::core::py_escape::escape_py_key;
use crate::core::transpiler::builder::PythonBuilder;
use crate::core::types::HoudiniNode;
use std::collections::{HashMap, HashSet};

pub fn write_creation_pass(
    builder: &mut PythonBuilder,
    nodes: &[Box<dyn HoudiniNode>],
    id_to_var: &HashMap<usize, String>,
    node_parent: &HashMap<usize, usize>,
    existing_nodes: &HashSet<usize>,
    existing_node_names: &HashMap<usize, String>,
) -> Result<(), String> {
    builder.line("# --- Node Creation Pass ---");
    for node in nodes {
        write_node_creation(
            builder,
            node.as_ref(),
            id_to_var,
            node_parent,
            existing_nodes,
            existing_node_names,
        )?;
    }

    Ok(())
}

fn write_node_creation(
    builder: &mut PythonBuilder,
    node: &dyn HoudiniNode,
    id_to_var: &HashMap<usize, String>,
    node_parent: &HashMap<usize, usize>,
    existing_nodes: &HashSet<usize>,
    existing_node_names: &HashMap<usize, String>,
) -> Result<(), String> {
    let node_id = node.get_id();
    let var_name = id_to_var
        .get(&node_id)
        .ok_or_else(|| format!("missing variable mapping for node id {}", node_id))?;

    let parent_var = get_parent_var(node_id, id_to_var, node_parent)?;
    let is_existing = existing_nodes.contains(&node_id);

    match (is_existing, parent_var) {
        (true, Some(p_var)) => {
            write_existing_node(builder, node, var_name, p_var, existing_node_names)
        }
        (true, None) => Err(format!(
            "existing node id {} has no parent mapping",
            node_id
        )),
        (false, Some(p_var)) => write_nested_node(builder, node, var_name, p_var),
        (false, None) => write_root_node(builder, node, var_name),
    }
}

fn get_parent_var<'a>(
    node_id: usize,
    id_to_var: &'a HashMap<usize, String>,
    node_parent: &HashMap<usize, usize>,
) -> Result<Option<&'a str>, String> {
    node_parent
        .get(&node_id)
        .map(|pid| {
            id_to_var
                .get(pid)
                .map(|s| s.as_str())
                .ok_or_else(|| format!("parent node id {} not found in variable mapping", pid))
        })
        .transpose()
}

fn write_existing_node(
    builder: &mut PythonBuilder,
    node: &dyn HoudiniNode,
    var_name: &str,
    parent_var: &str,
    existing_node_names: &HashMap<usize, String>,
) -> Result<(), String> {
    let original_name = existing_node_names
        .get(&node.get_id())
        .map(|s| s.as_str())
        .unwrap_or(node.get_name());

    builder.line(&format!(
        "{} = hou.node({}.path() + '/{}')",
        var_name,
        parent_var,
        escape_py_key(original_name)
    ));

    Ok(())
}

fn write_nested_node(
    builder: &mut PythonBuilder,
    node: &dyn HoudiniNode,
    var_name: &str,
    parent_var: &str,
) -> Result<(), String> {
    builder.line(&format!(
        "{} = {}.createNode('{}', '{}')",
        var_name,
        parent_var,
        node.get_node_type(),
        escape_py_key(node.get_name())
    ));

    Ok(())
}

fn write_root_node(
    builder: &mut PythonBuilder,
    node: &dyn HoudiniNode,
    var_name: &str,
) -> Result<(), String> {
    builder.line(&format!(
        "{} = parent.createNode('{}', '{}')",
        var_name,
        node.get_node_type(),
        escape_py_key(node.get_name())
    ));

    Ok(())
}
