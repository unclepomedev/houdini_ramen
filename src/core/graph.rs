use crate::core::py_escape::python_string_literal;
use crate::core::transpiler::Transpiler;
use crate::core::types::{ContainerType, HoudiniNode};

pub struct NodeGraph {
    parent_path: String,
    nodes: Vec<Box<dyn HoudiniNode>>,
    auto_create_type: Option<ContainerType>,
    auto_clear: bool,
    display_node_id: Option<usize>,
}

impl NodeGraph {
    pub fn new(parent_path: &str) -> Self {
        Self {
            parent_path: parent_path.to_string(),
            nodes: Vec::new(),
            auto_create_type: None,
            auto_clear: false,
            display_node_id: None,
        }
    }

    pub fn with_auto_create(mut self, container_type: ContainerType) -> Self {
        self.auto_create_type = Some(container_type);
        self
    }

    pub fn with_auto_clear(mut self) -> Self {
        self.auto_clear = true;
        self
    }

    /// Specify the node that will be the final output (display).
    pub fn set_display<T: HoudiniNode>(&mut self, node: &T) {
        self.display_node_id = Some(node.get_id());
    }

    /// Registers the node in the graph and returns it to the caller as is.
    pub fn add<T: HoudiniNode + Clone + 'static>(&mut self, node: T) -> T {
        self.nodes.push(Box::new(node.clone()));
        node
    }

    /// finish building the graph and internally call Transpiler to generate the script
    pub fn build(self) -> String {
        let mut transpiler =
            Transpiler::new(&self.parent_path, self.auto_create_type, self.auto_clear);

        if let Some(id) = self.display_node_id {
            transpiler.set_display_node(id);
        }

        let result = (|| -> Result<String, String> {
            for node in self.nodes {
                transpiler.add_boxed(node)?;
            }
            transpiler.generate_script()
        })();

        // TODO: Error handling is done here to avoid hassle, but for users who want to control the process themselves, the Result should be exposed.
        result.unwrap_or_else(|e| {
            eprintln!("Houdini Ramen Build Error: {}", e);

            let full_msg = format!("Houdini Ramen Error: {}", e);
            let safe_msg = python_string_literal(&full_msg);

            format!(
                "import hou\nhou.ui.displayMessage({}, severity=hou.severityType.Error)\nraise RuntimeError({})",
                safe_msg, safe_msg
            )
        })
    }
}
