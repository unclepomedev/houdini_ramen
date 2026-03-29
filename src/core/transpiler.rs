pub mod builder;
pub mod passes;

#[cfg(test)]
pub mod tests;

use crate::core::py_escape::sanitize_py_ident;
use crate::core::transpiler::builder::PythonBuilder;
use crate::core::types::{ContainerType, HoudiniNode};
use std::collections::HashMap;

pub struct Transpiler {
    parent_path: String,
    nodes: Vec<Box<dyn HoudiniNode>>,
    id_to_var: HashMap<usize, String>,
    auto_create_type: Option<ContainerType>,
    auto_clear: bool,
    display_node_id: Option<usize>,
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
        }
    }

    pub fn set_display_node(&mut self, id: usize) {
        self.display_node_id = Some(id);
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
        passes::creation::write_creation_pass(&mut builder, &self.nodes, &self.id_to_var)?;
        passes::spare_params::write_spare_parameter_pass(
            &mut builder,
            &self.nodes,
            &self.id_to_var,
        )?;
        passes::parameters::write_parameter_pass(&mut builder, &self.nodes, &self.id_to_var)?;
        passes::links::write_link_pass(&mut builder, &self.nodes, &self.id_to_var);

        let display_var = self
            .display_node_id
            .and_then(|id| self.id_to_var.get(&id))
            .map(|s| s.as_str());

        passes::footer::write_footer(&mut builder, display_var);

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
}
