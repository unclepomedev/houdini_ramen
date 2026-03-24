use crate::core::py_escape::python_string_literal;
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
    fn get_node_type(&self) -> &'static str;
    fn get_inputs(&self) -> &BTreeMap<usize, String>;
    fn get_params(&self) -> &HashMap<String, ParamValue>;
}

impl ParamValue {
    pub fn to_python_expr(&self) -> String {
        match self {
            ParamValue::Toggle(v) => {
                if *v {
                    "1".to_string()
                } else {
                    "0".to_string()
                }
            }
            ParamValue::Int(v) | ParamValue::Menu(v) => v.to_string(),
            ParamValue::Float(v) => format!("{:.4}", v),
            ParamValue::String(v) | ParamValue::Data(v) => python_string_literal(v),
            ParamValue::Int2(v) => format!("({}, {})", v[0], v[1]),
            ParamValue::Int3(v) => format!("({}, {}, {})", v[0], v[1], v[2]),
            ParamValue::Int4(v) => format!("({}, {}, {}, {})", v[0], v[1], v[2], v[3]),
            ParamValue::IntArray(v) => format!(
                "({})",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            ParamValue::Float2(v) => format!("({:.4}, {:.4})", v[0], v[1]),
            ParamValue::Float3(v) => format!("({:.4}, {:.4}, {:.4})", v[0], v[1], v[2]),
            ParamValue::Float4(v) => {
                format!("({:.4}, {:.4}, {:.4}, {:.4})", v[0], v[1], v[2], v[3])
            }
            ParamValue::FloatArray(v) => format!(
                "({})",
                v.iter()
                    .map(|x| format!("{:.4}", x))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            ParamValue::Button => "".to_string(), // button uses `pressButton()` instead of `set()`, so it's an empty string
            ParamValue::Ramp(_) => "None".to_string(), // TODO: Building Ramp structures using the Houdini API is complex and will be implemented later.
        }
    }

    pub fn is_tuple(&self) -> bool {
        matches!(
            self,
            ParamValue::Int2(_)
                | ParamValue::Int3(_)
                | ParamValue::Int4(_)
                | ParamValue::IntArray(_)
                | ParamValue::Float2(_)
                | ParamValue::Float3(_)
                | ParamValue::Float4(_)
                | ParamValue::FloatArray(_)
        )
    }

    pub fn is_trigger(&self) -> bool {
        matches!(self, ParamValue::Button)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_value_serialization() {
        assert_eq!(ParamValue::Toggle(true).to_python_expr(), "1");
        assert_eq!(ParamValue::Toggle(false).to_python_expr(), "0");

        assert_eq!(ParamValue::Int(42).to_python_expr(), "42");
        assert_eq!(ParamValue::Int3([1, 2, 3]).to_python_expr(), "(1, 2, 3)");
        assert!(ParamValue::Int3([1, 2, 3]).is_tuple());

        assert_eq!(
            ParamValue::Float(std::f32::consts::PI).to_python_expr(),
            "3.1416"
        );
        assert_eq!(
            ParamValue::Float3([1.0, 0.5, -0.25]).to_python_expr(),
            "(1.0000, 0.5000, -0.2500)"
        );

        assert_eq!(
            ParamValue::String("hello \"world\"".to_string()).to_python_expr(),
            "\"hello \\\"world\\\"\""
        );

        assert_eq!(
            ParamValue::String("C:\\path\\to\\file".to_string()).to_python_expr(),
            "\"C:\\\\path\\\\to\\\\file\""
        );

        assert_eq!(ParamValue::Button.to_python_expr(), "");
        assert!(ParamValue::Button.is_trigger());
    }
}
