use crate::core::types::param::ParamValue;
use crate::core::types::spare::SpareParam;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputPin {
    Index(usize),
    Name(String),
}

impl From<usize> for InputPin {
    fn from(idx: usize) -> Self {
        InputPin::Index(idx)
    }
}
impl From<&str> for InputPin {
    fn from(name: &str) -> Self {
        InputPin::Name(name.to_string())
    }
}
impl From<String> for InputPin {
    fn from(name: String) -> Self {
        InputPin::Name(name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputPin {
    Index(usize),
    Name(String),
}

impl From<usize> for OutputPin {
    fn from(idx: usize) -> Self {
        OutputPin::Index(idx)
    }
}
impl From<&str> for OutputPin {
    fn from(name: &str) -> Self {
        OutputPin::Name(name.to_string())
    }
}
impl From<String> for OutputPin {
    fn from(name: String) -> Self {
        OutputPin::Name(name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeOutput {
    pub node_id: usize,
    pub pin: OutputPin,
}

pub trait HoudiniNode {
    fn get_id(&self) -> usize;
    fn get_name(&self) -> &str;
    fn get_node_type(&self) -> &'static str;
    fn get_inputs(&self) -> &BTreeMap<InputPin, (usize, OutputPin)>;
    fn get_params(&self) -> &HashMap<String, ParamValue>;
    fn get_spare_params(&self) -> &[SpareParam] {
        &[]
    }
    fn get_dive_target(&self) -> Option<&'static str> {
        None
    }
}

impl<T: HoudiniNode> From<&T> for NodeOutput {
    fn from(node: &T) -> Self {
        NodeOutput {
            node_id: node.get_id(),
            pin: OutputPin::Index(0),
        }
    }
}

impl From<&NodeOutput> for NodeOutput {
    fn from(output: &NodeOutput) -> Self {
        output.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerType {
    Geo,
    DopNet,
    MatNet,
    Subnet,
    Cop2Net,
    ChopNet,
    LopNet,
    RopNet,
}

impl ContainerType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Geo => "geo",
            Self::DopNet => "dopnet",
            Self::MatNet => "matnet",
            Self::Subnet => "subnet",
            Self::Cop2Net => "cop2net",
            Self::ChopNet => "chopnet",
            Self::LopNet => "lopnet",
            Self::RopNet => "ropnet",
        }
    }
}
