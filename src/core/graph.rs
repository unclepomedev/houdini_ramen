use crate::core::context::Transpiler;
use crate::core::types::HoudiniNode;

pub struct NodeGraph {
    parent_path: String,
    nodes: Vec<Box<dyn HoudiniNode>>,
}

impl NodeGraph {
    pub fn new(parent_path: &str) -> Self {
        Self {
            parent_path: parent_path.to_string(),
            nodes: Vec::new(),
        }
    }

    pub fn add_node<T: HoudiniNode + Clone + 'static>(mut self, node: &T) -> Self {
        self.nodes.push(Box::new(node.clone()));
        self
    }

    /// finish building the graph and internally call Transpiler to generate the script
    pub fn build(self) -> String {
        let mut transpiler = Transpiler::new(&self.parent_path);
        for node in self.nodes {
            transpiler.add_boxed(node);
        }
        transpiler.generate_script()
    }
}
