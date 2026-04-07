use crate::core::types::NODE_ID_COUNTER;
use crate::core::types::param::ParamValue;
use crate::core::types::spare::SpareParam;
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::Ordering::Relaxed;

pub trait HoudiniNode {
    fn get_id(&self) -> usize;
    fn get_name(&self) -> &str;
    fn get_node_type(&self) -> &'static str;
    fn get_inputs(&self) -> &BTreeMap<usize, (usize, usize)>;
    fn get_params(&self) -> &HashMap<String, ParamValue>;
    fn get_spare_params(&self) -> &[SpareParam] {
        &[]
    }
    fn get_dive_target(&self) -> Option<&'static str> {
        None
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

#[derive(Debug, Clone)]
pub struct NodeBase {
    pub id: usize,
    pub name: String,
    pub inputs: BTreeMap<usize, (usize, usize)>,
    pub params: HashMap<String, ParamValue>,
    pub spare_params: Vec<SpareParam>,
    pub next_input_index: usize,
}

impl NodeBase {
    pub fn new(name: &str) -> Self {
        Self {
            id: NODE_ID_COUNTER.fetch_add(1, Relaxed),
            name: name.to_string(),
            inputs: BTreeMap::new(),
            params: HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
        }
    }
}
