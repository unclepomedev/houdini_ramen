use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, PartialEq)]
pub enum ParamValue {
    Toggle(bool),
    Int(i32),
    IntArray(Vec<i32>),
    Float(f32),
    FloatArray(Vec<f32>),
    String(String),
    Menu(i32),
    Button,
}

pub trait HoudiniNode {
    fn get_name(&self) -> &str;
    fn get_inputs(&self) -> &BTreeMap<usize, String>;
    fn get_params(&self) -> &HashMap<String, ParamValue>;
}
