use crate::core::py_escape::python_string_literal;
use crate::core::types::ramp::RampPoint;

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
    Expression(String),
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
            ParamValue::Expression(v) => python_string_literal(v),
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

    pub fn is_expression(&self) -> bool {
        matches!(self, ParamValue::Expression(_))
    }

    fn format_int_array(v: &[i32]) -> String {
        if v.len() == 1 {
            return format!("({},)", v[0]);
        }
        let items: Vec<String> = v.iter().map(|x| x.to_string()).collect();
        format!("({})", items.join(", "))
    }

    fn format_float_array(v: &[f32]) -> String {
        if v.len() == 1 {
            return format!("({:.4},)", v[0]);
        }
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
            .map(|p| p.interpolation.as_hou_str().to_string())
            .collect();

        let keys: Vec<String> = points
            .iter()
            .map(|p| format!("{:.4}", p.position))
            .collect();

        let values: Vec<String> = points
            .iter()
            .enumerate()
            .map(|(i, p)| {
                if is_color {
                    assert_eq!(
                        p.value.len(),
                        3,
                        "Color Ramp point at index {} must have exactly 3 elements, but found {}",
                        i,
                        p.value.len()
                    );
                    format!("({:.4}, {:.4}, {:.4})", p.value[0], p.value[1], p.value[2])
                } else {
                    assert_eq!(
                        p.value.len(),
                        1,
                        "Float Ramp point at index {} must have exactly 1 element, but found {}",
                        i,
                        p.value.len()
                    );
                    format!("{:.4}", p.value[0])
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::ramp::RampInterpolation;

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
    fn test_one_param_value_array_serialization() {
        assert_eq!(ParamValue::IntArray(vec![42]).to_python_expr(), "(42,)");
        assert_eq!(
            ParamValue::FloatArray(vec![1.0]).to_python_expr(),
            "(1.0000,)"
        );
    }

    #[test]
    fn test_ramp_serialization_valid() {
        let float_ramp = ParamValue::Ramp(vec![
            RampPoint {
                position: 0.0,
                value: vec![0.0],
                interpolation: RampInterpolation::Linear,
            },
            RampPoint {
                position: 1.0,
                value: vec![1.0],
                interpolation: RampInterpolation::CatmullRom,
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
                interpolation: RampInterpolation::Constant,
            },
            RampPoint {
                position: 1.0,
                value: vec![0.0, 0.0, 1.0],
                interpolation: RampInterpolation::Linear,
            },
        ]);
        assert_eq!(
            color_ramp.to_python_expr(),
            "hou.Ramp((hou.rampBasis.Constant, hou.rampBasis.Linear,), (0.0000, 1.0000,), ((1.0000, 0.0000, 0.0000), (0.0000, 0.0000, 1.0000),))"
        );

        let empty_ramp = ParamValue::Ramp(vec![]);
        assert_eq!(
            empty_ramp.to_python_expr(),
            "hou.Ramp((hou.rampBasis.Linear,), (0.0,), (0.0,))"
        );
    }

    #[test]
    #[should_panic(
        expected = "Color Ramp point at index 0 must have exactly 3 elements, but found 1"
    )]
    fn test_ramp_serialization_invalid_color_panics() {
        let invalid_mixed_ramp = ParamValue::Ramp(vec![
            RampPoint {
                position: 0.0,
                value: vec![1.0],
                interpolation: RampInterpolation::Linear,
            },
            RampPoint {
                position: 1.0,
                value: vec![0.5, 0.2, 0.8],
                interpolation: RampInterpolation::Linear,
            },
        ]);
        let _ = invalid_mixed_ramp.to_python_expr();
    }

    #[test]
    #[should_panic(
        expected = "Float Ramp point at index 0 must have exactly 1 element, but found 0"
    )]
    fn test_ramp_serialization_empty_value_panics() {
        let invalid_empty_val_ramp = ParamValue::Ramp(vec![RampPoint {
            position: 0.0,
            value: vec![],
            interpolation: RampInterpolation::Linear,
        }]);

        let _ = invalid_empty_val_ramp.to_python_expr();
    }

    #[test]
    #[should_panic(
        expected = "Color Ramp point at index 0 must have exactly 3 elements, but found 4"
    )]
    fn test_ramp_serialization_truncation_panics() {
        let invalid_color_ramp = ParamValue::Ramp(vec![RampPoint {
            position: 0.0,
            value: vec![1.0, 0.5, 0.2, 0.9],
            interpolation: RampInterpolation::Linear,
        }]);
        let _ = invalid_color_ramp.to_python_expr();
    }
}
