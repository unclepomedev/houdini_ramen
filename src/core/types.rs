use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, PartialEq)]
/// dedicated structure for representing Ramp (gradients and curves)
pub struct RampPoint {
    pub position: f32,
    pub value: Vec<f32>, // RampFloat has 1 element, RampColor has 3 elements.
    pub interpolation: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParamValue {
    Toggle(bool),

    Int(i32),
    Int2([i32; 2]),
    Int3([i32; 3]),
    Int4([i32; 4]),
    IntArray(Vec<i32>), // fallback for 5+ elements

    Float(f32),
    Float2([f32; 2]),
    Float3([f32; 3]), // color (RGB) and Vector3 (XYZ) are all protected here
    Float4([f32; 4]),
    FloatArray(Vec<f32>), // fallback for 5+ elements

    String(String),
    Data(String), // raw data such as VEX snippets
    Menu(i32),
    Button,
    Ramp(Vec<RampPoint>),
}

pub trait HoudiniNode {
    fn get_name(&self) -> &str;
    fn get_inputs(&self) -> &BTreeMap<usize, String>;
    fn get_params(&self) -> &HashMap<String, ParamValue>;
}
