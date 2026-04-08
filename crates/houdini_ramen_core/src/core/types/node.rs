use crate::core::types::param::ParamValue;
use crate::core::types::spare::SpareParam;
use std::collections::{BTreeMap, HashMap};

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
