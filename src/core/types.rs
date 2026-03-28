use crate::core::py_escape::python_string_literal;
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::AtomicUsize;

pub static NODE_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

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
    fn get_id(&self) -> usize;
    fn get_name(&self) -> &str;
    fn get_node_type(&self) -> &'static str;
    fn get_inputs(&self) -> &BTreeMap<usize, (usize, usize)>;
    fn get_params(&self) -> &HashMap<String, ParamValue>;
}

impl ParamValue {
    pub fn to_python_expr(&self) -> String {
        match self {
            ParamValue::Toggle(v) => (if *v { "1" } else { "0" }).to_string(),
            ParamValue::Int(v) | ParamValue::Menu(v) => v.to_string(),
            ParamValue::Float(v) => format!("{:.4}", v),
            ParamValue::String(v) | ParamValue::Data(v) => python_string_literal(v),
            ParamValue::Int2(v) => format!("({}, {})", v[0], v[1]),
            ParamValue::Int3(v) => format!("({}, {}, {})", v[0], v[1], v[2]),
            ParamValue::Int4(v) => format!("({}, {}, {}, {})", v[0], v[1], v[2], v[3]),
            ParamValue::IntArray(v) => Self::format_int_array(v),
            ParamValue::Float2(v) => format!("({:.4}, {:.4})", v[0], v[1]),
            ParamValue::Float3(v) => format!("({:.4}, {:.4}, {:.4})", v[0], v[1], v[2]),
            ParamValue::Float4(v) => {
                format!("({:.4}, {:.4}, {:.4}, {:.4})", v[0], v[1], v[2], v[3])
            }
            ParamValue::FloatArray(v) => Self::format_float_array(v),
            ParamValue::Button => "".to_string(), // button uses `pressButton()` instead of `set()`, so it's an empty string
            ParamValue::Ramp(points) => Self::format_ramp(points),
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

    fn format_int_array(v: &[i32]) -> String {
        let items: Vec<String> = v.iter().map(|x| x.to_string()).collect();
        format!("({})", items.join(", "))
    }

    fn format_float_array(v: &[f32]) -> String {
        let items: Vec<String> = v.iter().map(|x| format!("{:.4}", x)).collect();
        format!("({})", items.join(", "))
    }

    fn format_ramp(points: &[RampPoint]) -> String {
        if points.is_empty() {
            return "hou.Ramp((hou.rampBasis.Linear,), (0.0,), (0.0,))".to_string();
        }

        // Infer from the full ramp to avoid dropping RGB channels when points are mixed.
        let is_color = points.iter().any(|p| p.value.len() >= 3);

        let basis: Vec<String> = points
            .iter()
            .map(|p| Self::get_ramp_basis(p.interpolation).to_string())
            .collect();

        let keys: Vec<String> = points
            .iter()
            .map(|p| format!("{:.4}", p.position))
            .collect();

        let values: Vec<String> = points
            .iter()
            .map(|p| {
                if is_color {
                    let r = p.value.first().unwrap_or(&0.0);
                    let g = p.value.get(1).unwrap_or(&0.0);
                    let b = p.value.get(2).unwrap_or(&0.0);
                    format!("({:.4}, {:.4}, {:.4})", r, g, b)
                } else {
                    let v = p.value.first().unwrap_or(&0.0);
                    format!("{:.4}", v)
                }
            })
            .collect();

        format!(
            "hou.Ramp(({},), ({},), ({},))",
            basis.join(", "),
            keys.join(", "),
            values.join(", ")
        )
    }

    fn get_ramp_basis(interpolation: i32) -> &'static str {
        match interpolation {
            0 => "hou.rampBasis.Constant",
            1 => "hou.rampBasis.Linear",
            2 => "hou.rampBasis.CatmullRom",
            3 => "hou.rampBasis.MonotoneCubic",
            4 => "hou.rampBasis.Bezier",
            5 => "hou.rampBasis.BSpline",
            6 => "hou.rampBasis.Hermite",
            _ => "hou.rampBasis.Linear",
        }
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

    #[test]
    fn test_ramp_serialization() {
        let float_ramp = ParamValue::Ramp(vec![
            RampPoint {
                position: 0.0,
                value: vec![0.0],
                interpolation: 1,
            },
            RampPoint {
                position: 1.0,
                value: vec![1.0],
                interpolation: 2,
            },
        ]);
        assert_eq!(
            float_ramp.to_python_expr(),
            "hou.Ramp((hou.rampBasis.Linear, hou.rampBasis.CatmullRom,), (0.0000, 1.0000,), (0.0000, 1.0000,))"
        );

        let color_ramp = ParamValue::Ramp(vec![
            RampPoint {
                position: 0.0,
                value: vec![1.0, 0.0, 0.0],
                interpolation: 0,
            },
            RampPoint {
                position: 1.0,
                value: vec![0.0, 0.0, 1.0],
                interpolation: 1,
            },
        ]);
        assert_eq!(
            color_ramp.to_python_expr(),
            "hou.Ramp((hou.rampBasis.Constant, hou.rampBasis.Linear,), (0.0000, 1.0000,), ((1.0000, 0.0000, 0.0000), (0.0000, 0.0000, 1.0000),))"
        );

        let mixed_ramp = ParamValue::Ramp(vec![
            RampPoint {
                position: 0.0,
                value: vec![1.0],
                interpolation: 1,
            },
            RampPoint {
                position: 1.0,
                value: vec![0.5, 0.2, 0.8],
                interpolation: 1,
            },
        ]);
        assert_eq!(
            mixed_ramp.to_python_expr(),
            "hou.Ramp((hou.rampBasis.Linear, hou.rampBasis.Linear,), (0.0000, 1.0000,), ((1.0000, 0.0000, 0.0000), (0.5000, 0.2000, 0.8000),))"
        );

        let empty_ramp = ParamValue::Ramp(vec![]);
        assert_eq!(
            empty_ramp.to_python_expr(),
            "hou.Ramp((hou.rampBasis.Linear,), (0.0,), (0.0,))"
        );
    }
}
