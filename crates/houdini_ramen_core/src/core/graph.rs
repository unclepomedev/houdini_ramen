use crate::core::py_escape::python_string_literal;
use crate::core::transpiler::Transpiler;
use crate::core::types::ParamValue;
use crate::core::types::{ContainerType, HoudiniNode, InputPin, NodeOutput, OutputPin};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::LazyLock;

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
pub struct InnerGraph<'a, C> {
    graph: &'a mut NodeGraph,
    container_id: usize,
    _phantom: PhantomData<C>,
}

impl<'a, C> InnerGraph<'a, C> {
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

    /// Adds a new node inside the container.
    pub fn add<T: HoudiniNode + Clone + 'static>(&mut self, node: T) -> T {
        self.graph
            .nested_nodes
            .push((Box::new(node.clone()), self.container_id));
        node
    }

    /// Sets the display flag for a node inside this container.
    /// Only `output.node_id` is used; `output.pin` is ignored.
    pub fn set_display<O: Into<NodeOutput>>(&mut self, output: O) {
        self.graph
            .nested_display_nodes
            .push((output.into().node_id, self.container_id));
    }

    pub fn connect_existing<I: Into<InputPin>, O: Into<NodeOutput>>(
        &mut self,
        dst: &ExistingNodeRef,
        input_pin: I,
        output: O,
    ) {
        let out = output.into();
        let found = self
            .graph
            .existing_nodes
            .iter_mut()
            .find(|(n, cid, _)| *cid == self.container_id && n.id == dst.id);
        if let Some((node, _, _)) = found {
            node.inputs.insert(input_pin.into(), (out.node_id, out.pin));
        } else {
            panic!(
                "Houdini Ramen Error: Attempted to wire to ExistingNodeRef '{}' which does not belong to the current container.",
                dst.name
            );
        }
    }

    pub fn dive_into<T: HoudiniNode, F, R>(&mut self, container: &T, f: F) -> R
    where
        F: FnOnce(&mut InnerGraph<T>) -> R,
    {
        let mut nested_inner = InnerGraph {
            graph: self.graph,
            container_id: container.get_id(),
            _phantom: PhantomData,
        };
        f(&mut nested_inner)
    }
}

/// A lightweight reference to a pre-existing node inside a subnet.
#[derive(Debug, Clone)]
pub struct ExistingNodeRef {
    pub id: usize,
    pub name: String,
    pub inputs: BTreeMap<InputPin, (usize, OutputPin)>,
}

impl ExistingNodeRef {
    pub fn typed_as<N: HoudiniNode>(self) -> TypedExistingNodeRef<N> {
        TypedExistingNodeRef {
            base: self,
            _phantom: PhantomData,
        }
    }
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
    fn get_inputs(&self) -> &BTreeMap<InputPin, (usize, OutputPin)> {
        &self.inputs
    }
    fn get_params(&self) -> &HashMap<String, ParamValue> {
        static EMPTY: LazyLock<HashMap<String, ParamValue>> = LazyLock::new(HashMap::new);
        &EMPTY
    }
}

#[derive(Debug, Clone)]
pub struct TypedExistingNodeRef<N> {
    pub base: ExistingNodeRef,
    _phantom: PhantomData<N>,
}

impl<N> Deref for TypedExistingNodeRef<N> {
    type Target = ExistingNodeRef;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<N> HoudiniNode for TypedExistingNodeRef<N> {
    fn get_id(&self) -> usize {
        self.base.get_id()
    }
    fn get_name(&self) -> &str {
        self.base.get_name()
    }
    fn get_node_type(&self) -> &'static str {
        self.base.get_node_type()
    }
    fn get_inputs(&self) -> &BTreeMap<InputPin, (usize, OutputPin)> {
        self.base.get_inputs()
    }
    fn get_params(&self) -> &HashMap<String, ParamValue> {
        self.base.get_params()
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
    /// Only `output.node_id` is used; `output.pin` is ignored.
    pub fn set_display<O: Into<NodeOutput>>(&mut self, output: O) {
        self.display_node_id = Some(output.into().node_id);
    }

    /// Registers the node in the graph and returns it to the caller as is.
    pub fn add<T: HoudiniNode + Clone + 'static>(&mut self, node: T) -> T {
        self.nodes.push(Box::new(node.clone()));
        node
    }

    /// Dive into a container node to add inner nodes.
    pub fn dive_into<T: HoudiniNode, F, R>(&mut self, container: &T, f: F) -> R
    where
        F: FnOnce(&mut InnerGraph<T>) -> R,
    {
        let mut inner = InnerGraph {
            graph: self,
            container_id: container.get_id(),
            _phantom: PhantomData,
        };
        f(&mut inner)
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
        result.unwrap_or_else(|e| {
            eprintln!("Houdini Ramen Build Error: {}", e);
            let full_msg = format!("Houdini Ramen Error: {}", e);
            let safe_msg = python_string_literal(&full_msg);
            format!("import hou\nhou.ui.displayMessage({}, severity=hou.severityType.Error)\nraise RuntimeError({})", safe_msg, safe_msg)
        })
    }
}

pub struct NodeWiring<'a, 'g, C, N> {
    inner: &'a mut InnerGraph<'g, C>,
    target_id: usize,
    target_name: String,
    _phantom: PhantomData<N>,
}

impl<'a, 'g, C> InnerGraph<'g, C> {
    pub fn wire<N>(&'a mut self, target: &TypedExistingNodeRef<N>) -> NodeWiring<'a, 'g, C, N> {
        NodeWiring {
            inner: self,
            target_id: target.get_id(),
            target_name: target.get_name().to_string(),
            _phantom: PhantomData,
        }
    }

    pub fn wire_any(&'a mut self, target: &ExistingNodeRef) -> NodeWiring<'a, 'g, C, ()> {
        NodeWiring {
            inner: self,
            target_id: target.get_id(),
            target_name: target.get_name().to_string(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, 'g, C, N> NodeWiring<'a, 'g, C, N> {
    pub fn set_input_at<O: Into<NodeOutput>>(mut self, index: usize, output: O) -> Self {
        self.connect_internal(InputPin::Index(index), output);
        self
    }

    pub fn set_input_name<O: Into<NodeOutput>>(mut self, name: &str, output: O) -> Self {
        self.connect_internal(InputPin::Name(name.to_string()), output);
        self
    }

    fn connect_internal<O: Into<NodeOutput>>(&mut self, input_pin: InputPin, output: O) {
        let out = output.into();
        let found = self
            .inner
            .graph
            .existing_nodes
            .iter_mut()
            .find(|(n, cid, _)| *cid == self.inner.container_id && n.id == self.target_id);

        if let Some((node, _, _)) = found {
            node.inputs.insert(input_pin, (out.node_id, out.pin));
        } else {
            panic!(
                "Houdini Ramen Error: Attempted to wire to ExistingNodeRef '{}' which does not belong to the current container.",
                self.target_name
            );
        }
    }
}
