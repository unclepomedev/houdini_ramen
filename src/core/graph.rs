use crate::core::py_escape::python_string_literal;
use crate::core::transpiler::Transpiler;
use crate::core::types::{ContainerType, HoudiniNode};

pub struct NodeGraph {
    parent_path: String,
    nodes: Vec<Box<dyn HoudiniNode>>,
    auto_create_type: Option<ContainerType>,
    auto_clear: bool,
    display_node_id: Option<usize>,
    /// Nodes that belong to a specific parent node (nested inside a subnet)
    nested_nodes: Vec<(Box<dyn HoudiniNode>, usize)>,
    /// Existing nodes inside subnets (fetched, not created)
    existing_nodes: Vec<(Box<dyn HoudiniNode>, usize, String)>,
    /// Display flags for subnet inner graphs: (display_node_id, parent_node_id)
    nested_display_nodes: Vec<(usize, usize)>,
}

/// A scoped inner graph used inside `dive_into` to add nodes under a container node.
pub struct InnerGraph<'a> {
    graph: &'a mut NodeGraph,
    container_id: usize,
}

impl<'a> InnerGraph<'a> {
    /// Registers a pre-existing node inside the container (e.g., "Prev_Frame", "Input_1").
    /// These nodes are fetched via `hou.item(...)` rather than created.
    pub fn get_existing_node(&mut self, name: &str) -> ExistingNodeRef {
        use crate::core::types::NODE_ID_COUNTER;
        use std::sync::atomic::Ordering;

        let id = NODE_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let node = ExistingNodeRef {
            id,
            name: name.to_string(),
        };
        self.graph.existing_nodes.push((
            Box::new(node.clone()),
            self.container_id,
            name.to_string(),
        ));
        node
    }

    /// Adds a new node inside the container.
    pub fn add<T: HoudiniNode + Clone + 'static>(&mut self, node: T) -> T {
        self.graph
            .nested_nodes
            .push((Box::new(node.clone()), self.container_id));
        node
    }

    /// Sets the display flag for a node inside this container.
    pub fn set_display<T: HoudiniNode>(&mut self, node: &T) {
        self.graph
            .nested_display_nodes
            .push((node.get_id(), self.container_id));
    }
}

/// A lightweight reference to a pre-existing node inside a subnet.
#[derive(Debug, Clone)]
pub struct ExistingNodeRef {
    pub id: usize,
    pub name: String,
}

impl HoudiniNode for ExistingNodeRef {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        ""
    }
    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        static EMPTY: std::collections::BTreeMap<usize, (usize, usize)> = std::collections::BTreeMap::new();
        &EMPTY
    }
    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        static EMPTY: std::sync::LazyLock<std::collections::HashMap<String, crate::core::types::ParamValue>> =
            std::sync::LazyLock::new(std::collections::HashMap::new);
        &EMPTY
    }
}

impl NodeGraph {
    pub fn new(parent_path: &str) -> Self {
        Self {
            parent_path: parent_path.to_string(),
            nodes: Vec::new(),
            auto_create_type: None,
            auto_clear: false,
            display_node_id: None,
            nested_nodes: Vec::new(),
            existing_nodes: Vec::new(),
            nested_display_nodes: Vec::new(),
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

    /// Dive into a container node to add inner nodes.
    pub fn dive_into<T: HoudiniNode, F>(&mut self, container: &T, f: F)
    where
        F: FnOnce(&mut InnerGraph),
    {
        let mut inner = InnerGraph {
            graph: self,
            container_id: container.get_id(),
        };
        f(&mut inner);
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
            for (node, parent_id, original_name) in self.existing_nodes {
                transpiler.add_existing_node(node, parent_id, &original_name)?;
            }
            for (node, parent_id) in self.nested_nodes {
                transpiler.add_boxed_with_parent(node, parent_id)?;
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
