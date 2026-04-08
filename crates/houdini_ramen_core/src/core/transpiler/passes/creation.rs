use crate::core::py_escape::escape_py_key;
use crate::core::transpiler::builder::PythonBuilder;
use crate::core::types::HoudiniNode;
use std::collections::{BTreeSet, HashMap, HashSet};

pub fn write_creation_pass(
    builder: &mut PythonBuilder,
    nodes: &[Box<dyn HoudiniNode>],
    id_to_var: &HashMap<usize, String>,
    node_parent: &HashMap<usize, usize>,
    existing_nodes: &HashSet<usize>,
    existing_node_names: &HashMap<usize, String>,
) -> Result<BTreeSet<String>, String> {
    builder.line("# --- Node Creation Pass ---");

    let id_to_node: HashMap<usize, &dyn HoudiniNode> =
        nodes.iter().map(|n| (n.get_id(), n.as_ref())).collect();

    let mut container_exprs = BTreeSet::new();

    let mut ctx = CreationContext {
        id_to_var,
        node_parent,
        existing_nodes,
        existing_node_names,
        id_to_node: &id_to_node,
        container_exprs: &mut container_exprs,
    };

    let sorted = topological_sort(nodes, node_parent)?;
    for idx in sorted {
        write_node_creation(builder, nodes[idx].as_ref(), &mut ctx)?;
    }

    Ok(container_exprs)
}

fn parent_depth(node_id: usize, node_parent: &HashMap<usize, usize>) -> Result<usize, String> {
    let mut depth = 0;
    let mut current = node_id;
    let mut visited = HashSet::new();
    while let Some(&pid) = node_parent.get(&current) {
        if !visited.insert(pid) {
            return Err(format!(
                "cycle detected in parent chain for node id {}",
                node_id
            ));
        }
        depth += 1;
        current = pid;
    }
    Ok(depth)
}

fn topological_sort(
    nodes: &[Box<dyn HoudiniNode>],
    node_parent: &HashMap<usize, usize>,
) -> Result<Vec<usize>, String> {
    let node_ids: HashSet<usize> = nodes.iter().map(|n| n.get_id()).collect();
    for (&child_id, &pid) in node_parent.iter() {
        if node_ids.contains(&child_id) && !node_ids.contains(&pid) {
            return Err(format!(
                "node id {} references missing parent id {}",
                child_id, pid
            ));
        }
    }

    let mut indices: Vec<usize> = (0..nodes.len()).collect();
    let depths: Vec<Result<usize, String>> = nodes
        .iter()
        .map(|n| parent_depth(n.get_id(), node_parent))
        .collect();

    for d in &depths {
        if let Err(e) = d {
            return Err(e.clone());
        }
    }

    indices.sort_by_key(|&i| *depths[i].as_ref().unwrap());
    Ok(indices)
}

struct CreationContext<'a> {
    id_to_var: &'a HashMap<usize, String>,
    node_parent: &'a HashMap<usize, usize>,
    existing_nodes: &'a HashSet<usize>,
    existing_node_names: &'a HashMap<usize, String>,
    id_to_node: &'a HashMap<usize, &'a dyn HoudiniNode>,
    container_exprs: &'a mut BTreeSet<String>,
}

fn write_node_creation(
    builder: &mut PythonBuilder,
    node: &dyn HoudiniNode,
    ctx: &mut CreationContext,
) -> Result<(), String> {
    let node_id = node.get_id();
    let var_name = ctx
        .id_to_var
        .get(&node_id)
        .ok_or_else(|| format!("missing variable mapping for node id {}", node_id))?;

    let parent_var = get_parent_var(node_id, ctx.id_to_var, ctx.node_parent)?;
    let parent_node = ctx
        .node_parent
        .get(&node_id)
        .and_then(|pid| ctx.id_to_node.get(pid).copied());

    let is_existing = ctx.existing_nodes.contains(&node_id);

    match (is_existing, parent_var) {
        (true, Some(p_var)) => {
            write_existing_node(builder, node, var_name, p_var, ctx.existing_node_names)
        }
        (true, None) => Err(format!(
            "existing node id {} has no parent mapping",
            node_id
        )),
        (false, Some(p_var)) => write_nested_node(
            builder,
            node,
            var_name,
            p_var,
            parent_node,
            ctx.container_exprs,
        ),
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

    let escaped = escape_py_key(original_name);
    builder.line(&format!(
        "{} = hou.node({}.path() + '/{}')",
        var_name, parent_var, escaped
    ));
    builder.line(&format!("if not {}:", var_name));
    builder.indent();
    builder.line(&format!(
        "raise RuntimeError(f\"Existing node not found: {{{}.path()}}\" + '/{}')",
        parent_var, escaped
    ));
    builder.dedent();

    Ok(())
}

fn write_nested_node(
    builder: &mut PythonBuilder,
    node: &dyn HoudiniNode,
    var_name: &str,
    parent_var: &str,
    parent_node: Option<&dyn HoudiniNode>,
    container_exprs: &mut BTreeSet<String>,
) -> Result<(), String> {
    let actual_parent_expr = if let Some(p) = parent_node {
        if let Some(target) = p.get_dive_target() {
            format!("{}.node('{}')", parent_var, escape_py_key(target))
        } else {
            parent_var.to_string()
        }
    } else {
        parent_var.to_string()
    };

    container_exprs.insert(actual_parent_expr.clone());

    builder.line(&format!(
        "{} = {}.createNode('{}', '{}')",
        var_name,
        actual_parent_expr,
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
