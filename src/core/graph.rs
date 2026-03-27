use crate::core::context::Transpiler;
use crate::core::types::HoudiniNode;

pub struct NodeGraph {
    parent_path: String,
    nodes: Vec<Box<dyn HoudiniNode>>,
    auto_create: bool,
    auto_clear: bool,
}

impl NodeGraph {
    pub fn new(parent_path: &str) -> Self {
        Self {
            parent_path: parent_path.to_string(),
            nodes: Vec::new(),
            auto_create: false,
            auto_clear: false,
        }
    }

    pub fn with_auto_create(mut self) -> Self {
        self.auto_create = true;
        self
    }

    pub fn with_auto_clear(mut self) -> Self {
        self.auto_clear = true;
        self
    }

    pub fn add_node<T: HoudiniNode + Clone + 'static>(mut self, node: &T) -> Self {
        self.nodes.push(Box::new(node.clone()));
        self
    }

    /// finish building the graph and internally call Transpiler to generate the script
    pub fn build(self) -> String {
        let mut transpiler = Transpiler::new(&self.parent_path, self.auto_create, self.auto_clear);
        for node in self.nodes {
            transpiler.add_boxed(node);
        }
        transpiler.generate_script()
    }
}
