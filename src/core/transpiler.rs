pub mod builder;
pub mod passes;

#[cfg(test)]
pub mod tests;

use crate::core::py_escape::sanitize_py_ident;
use crate::core::transpiler::builder::PythonBuilder;
use crate::core::types::{ContainerType, HoudiniNode};
use std::collections::{HashMap, HashSet};

pub struct Transpiler {
    parent_path: String,
    nodes: Vec<Box<dyn HoudiniNode>>,
    id_to_var: HashMap<usize, String>,
    auto_create_type: Option<ContainerType>,
    auto_clear: bool,
    display_node_id: Option<usize>,
    /// Display flags for nodes inside subnets: (display_node_id, container_node_id)
    nested_display_nodes: Vec<(usize, usize)>,
    /// Maps a node id to its parent node id (for nested/subnet nodes)
    node_parent: HashMap<usize, usize>,
    /// Set of node ids that are pre-existing (fetched via hou.item, not created)
    existing_nodes: HashSet<usize>,
    /// Maps a node id to the original name used for hou.item lookup
    existing_node_names: HashMap<usize, String>,
}

impl Transpiler {
    pub fn new(
        parent_path: &str,
        auto_create_type: Option<ContainerType>,
        auto_clear: bool,
    ) -> Self {
        Self {
            parent_path: parent_path.to_string(),
            nodes: Vec::new(),
            id_to_var: HashMap::new(),
            auto_create_type,
            auto_clear,
            display_node_id: None,
            nested_display_nodes: Vec::new(),
            node_parent: HashMap::new(),
            existing_nodes: HashSet::new(),
            existing_node_names: HashMap::new(),
        }
    }

    pub fn set_display_node(&mut self, id: usize) {
        self.display_node_id = Some(id);
    }

    pub fn add_nested_display_nodes(&mut self, nodes: Vec<(usize, usize)>) {
        self.nested_display_nodes.extend(nodes);
    }

    pub fn add<T: HoudiniNode + 'static>(&mut self, node: T) -> Result<(), String> {
        self.register_node(&node)?;
        self.nodes.push(Box::new(node));
        Ok(())
    }

    pub fn generate_script(&self) -> Result<String, String> {
        let mut builder = PythonBuilder::new();
        passes::header::write_header(
            &mut builder,
            &self.parent_path,
            self.auto_create_type,
            self.auto_clear,
        );
        let container_exprs = passes::creation::write_creation_pass(
            &mut builder,
            &self.nodes,
            &self.id_to_var,
            &self.node_parent,
            &self.existing_nodes,
            &self.existing_node_names,
        )?;
        passes::spare_params::write_spare_parameter_pass(
            &mut builder,
            &self.nodes,
            &self.id_to_var,
        )?;
        passes::parameters::write_parameter_pass(&mut builder, &self.nodes, &self.id_to_var)?;
        passes::links::write_link_pass(&mut builder, &self.nodes, &self.id_to_var);

        let display_var = if let Some(id) = self.display_node_id {
            Some(
                self.id_to_var
                    .get(&id)
                    .ok_or_else(|| format!("display node id {} not found", id))?
                    .as_str(),
            )
        } else {
            None
        };

        let nested_display_vars: Vec<&str> = self
            .nested_display_nodes
            .iter()
            .filter_map(|(nid, _cid)| self.id_to_var.get(nid).map(|v| v.as_str()))
            .collect();
        passes::footer::write_footer(
            &mut builder,
            display_var,
            &nested_display_vars,
            &container_exprs,
        );

        Ok(builder.build())
    }

    fn register_node(&mut self, node: &dyn HoudiniNode) -> Result<(), String> {
        let safe_name = sanitize_py_ident(node.get_name());
        let var_name = format!("n_{}_{}", safe_name, node.get_id());
        if self.id_to_var.insert(node.get_id(), var_name).is_some() {
            return Err(format!(
                "duplicate node id {} while registering '{}'",
                node.get_id(),
                node.get_name()
            ));
        }
        Ok(())
    }

    pub(crate) fn add_boxed(&mut self, node: Box<dyn HoudiniNode>) -> Result<(), String> {
        self.register_node(node.as_ref())?;
        self.nodes.push(node);
        Ok(())
    }

    pub(crate) fn add_boxed_with_parent(
        &mut self,
        node: Box<dyn HoudiniNode>,
        parent_node_id: usize,
    ) -> Result<(), String> {
        self.register_node(node.as_ref())?;
        self.node_parent.insert(node.get_id(), parent_node_id);
        self.nodes.push(node);
        Ok(())
    }

    pub(crate) fn add_existing_node(
        &mut self,
        node: Box<dyn HoudiniNode>,
        parent_node_id: usize,
        original_name: &str,
    ) -> Result<(), String> {
        self.register_node(node.as_ref())?;
        let id = node.get_id();
        self.node_parent.insert(id, parent_node_id);
        self.existing_nodes.insert(id);
        self.existing_node_names
            .insert(id, original_name.to_string());
        self.nodes.push(node);
        Ok(())
    }
}
