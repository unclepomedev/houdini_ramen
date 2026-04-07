use crate::core::py_escape::python_string_literal;
use crate::core::transpiler::Transpiler;
use crate::core::types::{ContainerType, HoudiniNode};
use std::collections::BTreeMap;

pub struct NodeGraph {
    parent_path: String,
    nodes: Vec<Box<dyn HoudiniNode>>,
    auto_create_type: Option<ContainerType>,
    auto_clear: bool,
    display_node_id: Option<usize>,
    /// Nodes that belong to a specific parent node (nested inside a subnet)
    nested_nodes: Vec<(Box<dyn HoudiniNode>, usize)>,
    /// Existing nodes inside subnets (fetched, not created)
    existing_nodes: Vec<(ExistingNodeRef, usize, String)>,
    /// Display flags for subnet inner graphs: (display_node_id, parent_node_id)
    nested_display_nodes: Vec<(usize, usize)>,
}

/// A scoped inner graph used inside `dive_into` to add nodes under a container node.
pub struct InnerGraph<'a> {
    graph: &'a mut NodeGraph,
    container_id: usize,
}

impl<'a> InnerGraph<'a> {
    /// Generate a reference to an existing node.
    pub fn existing_node(&mut self, name: &str) -> ExistingNodeRef {
        if let Some((node, _, _)) = self
            .graph
            .existing_nodes
            .iter()
            .find(|(_, cid, n)| *cid == self.container_id && n == name)
        {
            return node.clone();
        }
        use crate::core::types::NODE_ID_COUNTER;
        use std::sync::atomic::Ordering;

        let id = NODE_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let node = ExistingNodeRef {
            id,
            name: name.to_string(),
            inputs: BTreeMap::new(),
        };
        self.graph
            .existing_nodes
            .push((node.clone(), self.container_id, name.to_string()));
        node
    }

    /// Connect to the input port of an existing node.
    pub fn wire_to_existing<N: HoudiniNode>(&mut self, name: &str, input_idx: usize, target: &N) {
        self.existing_node(name);
        if let Some((node, _, _)) = self
            .graph
            .existing_nodes
            .iter_mut()
            .find(|(_, cid, n)| *cid == self.container_id && n == name)
        {
            node.inputs.insert(input_idx, (target.get_id(), 0));
        }
    }

    /// Specify the output port and connect the wire.
    pub fn wire_to_existing_from<N: HoudiniNode>(
        &mut self,
        name: &str,
        input_idx: usize,
        target: &N,
        output_idx: usize,
    ) {
        self.existing_node(name);
        if let Some((node, _, _)) = self
            .graph
            .existing_nodes
            .iter_mut()
            .find(|(_, cid, n)| *cid == self.container_id && n == name)
        {
            node.inputs.insert(input_idx, (target.get_id(), output_idx));
        }
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

    /// Receives a reference to a node and connects it as a target.
    pub fn connect_existing<N: HoudiniNode>(
        &mut self,
        dst: &ExistingNodeRef,
        input_idx: usize,
        src: &N,
    ) {
        if let Some((node, _, _)) = self
            .graph
            .existing_nodes
            .iter_mut()
            .find(|(n, cid, _)| *cid == self.container_id && n.id == dst.id)
        {
            node.inputs.insert(input_idx, (src.get_id(), 0));
        }
    }

    /// Specify the output port and connect the wires.
    pub fn connect_existing_from<N: HoudiniNode>(
        &mut self,
        dst: &ExistingNodeRef,
        input_idx: usize,
        src: &N,
        output_idx: usize,
    ) {
        if let Some((node, _, _)) = self
            .graph
            .existing_nodes
            .iter_mut()
            .find(|(n, cid, _)| *cid == self.container_id && n.id == dst.id)
        {
            node.inputs.insert(input_idx, (src.get_id(), output_idx));
        }
    }
}

/// A lightweight reference to a pre-existing node inside a subnet.
#[derive(Debug, Clone)]
pub struct ExistingNodeRef {
    pub id: usize,
    pub name: String,
    pub inputs: BTreeMap<usize, (usize, usize)>,
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
    fn get_inputs(&self) -> &BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }
    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        static EMPTY: std::sync::LazyLock<
            std::collections::HashMap<String, crate::core::types::ParamValue>,
        > = std::sync::LazyLock::new(std::collections::HashMap::new);
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

        if !self.nested_display_nodes.is_empty() {
            transpiler.add_nested_display_nodes(self.nested_display_nodes);
        }

        let result = (|| -> Result<String, String> {
            for node in self.nodes {
                transpiler.add_boxed(node)?;
            }
            for (node, parent_id) in self.nested_nodes {
                transpiler.add_boxed_with_parent(node, parent_id)?;
            }
            for (node, parent_id, original_name) in self.existing_nodes {
                transpiler.add_existing_node(Box::new(node), parent_id, &original_name)?;
            }
            transpiler.generate_script()
        })();

        // TODO: Error handling is done here to avoid hassle, but for users who want to control the process themselves, the Result should be exposed.
        result.unwrap_or_else(|e| {
            eprintln!("Houdini Ramen Build Error: {}", e);
            let full_msg = format!("Houdini Ramen Error: {}", e);
            let safe_msg = python_string_literal(&full_msg);
            format!("import hou\nhou.ui.displayMessage({}, severity=hou.severityType.Error)\nraise RuntimeError({})", safe_msg, safe_msg)
        })
    }
}
