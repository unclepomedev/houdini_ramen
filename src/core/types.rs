use crate::core::py_escape::python_string_literal;
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::AtomicUsize;

pub static NODE_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputPort {
    Index(usize),
    Name(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RampInterpolation {
    Constant = 0,
    Linear = 1,
    CatmullRom = 2,
    MonotoneCubic = 3,
    Bezier = 4,
    BSpline = 5,
    Hermite = 6,
}
impl RampInterpolation {
    pub fn as_hou_str(&self) -> &'static str {
        match self {
            Self::Constant => "hou.rampBasis.Constant",
            Self::Linear => "hou.rampBasis.Linear",
            Self::CatmullRom => "hou.rampBasis.CatmullRom",
            Self::MonotoneCubic => "hou.rampBasis.MonotoneCubic",
            Self::Bezier => "hou.rampBasis.Bezier",
            Self::BSpline => "hou.rampBasis.BSpline",
            Self::Hermite => "hou.rampBasis.Hermite",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// dedicated structure for representing Ramp (gradients and curves)
pub struct RampPoint {
    pub position: f32,
    pub value: Vec<f32>, // RampFloat has 1 element, RampColor has 3 elements.
    pub interpolation: RampInterpolation,
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
    Expression(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpareParam {
    Float {
        name: String,
        label: String,
        default: f32,
        min: f32,
        max: f32,
    },
    Int {
        name: String,
        label: String,
        default: i32,
        min: i32,
        max: i32,
    },
    String {
        name: String,
        label: String,
        default: String,
    },
    Toggle {
        name: String,
        label: String,
        default: bool,
    },
    Color {
        name: String,
        label: String,
        default: [f32; 3],
    },
    Button {
        name: String,
        label: String,
    },
    Menu {
        name: String,
        label: String,
        default: i32,
        menu_items: Vec<(String, String)>,
    },
    File {
        name: String,
        label: String,
        default: String,
    },
    NodePath {
        name: String,
        label: String,
        default: String,
    },
    RampFloat {
        name: String,
        label: String,
    },
    RampColor {
        name: String,
        label: String,
    },
}

pub struct SpareFloat {
    name: String,
    label: String,
    default: f32,
    min: f32,
    max: f32,
}
impl SpareFloat {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            default: 0.0,
            min: 0.0,
            max: 1.0,
        }
    }
    pub fn with_default(mut self, val: f32) -> Self {
        self.default = val;
        self
    }
    pub fn with_range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }
}
impl From<SpareFloat> for SpareParam {
    fn from(v: SpareFloat) -> Self {
        assert!(
            v.min <= v.max,
            "SpareFloat '{}' has invalid range: min {} > max {}",
            v.name,
            v.min,
            v.max
        );
        assert!(
            v.default >= v.min && v.default <= v.max,
            "SpareFloat '{}' default {} is out of range [{}, {}]",
            v.name,
            v.default,
            v.min,
            v.max
        );
        SpareParam::Float {
            name: v.name,
            label: v.label,
            default: v.default,
            min: v.min,
            max: v.max,
        }
    }
}

pub struct SpareInt {
    name: String,
    label: String,
    default: i32,
    min: i32,
    max: i32,
}
impl SpareInt {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            default: 0,
            min: 0,
            max: 10,
        }
    }
    pub fn with_default(mut self, val: i32) -> Self {
        self.default = val;
        self
    }
    pub fn with_range(mut self, min: i32, max: i32) -> Self {
        self.min = min;
        self.max = max;
        self
    }
}
impl From<SpareInt> for SpareParam {
    fn from(v: SpareInt) -> Self {
        assert!(
            v.min <= v.max,
            "SpareInt '{}' has invalid range: min {} > max {}",
            v.name,
            v.min,
            v.max
        );
        assert!(
            v.default >= v.min && v.default <= v.max,
            "SpareInt '{}' default {} is out of range [{}, {}]",
            v.name,
            v.default,
            v.min,
            v.max
        );
        SpareParam::Int {
            name: v.name,
            label: v.label,
            default: v.default,
            min: v.min,
            max: v.max,
        }
    }
}

pub struct SpareString {
    name: String,
    label: String,
    default: String,
}
impl SpareString {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            default: String::new(),
        }
    }
    pub fn with_default(mut self, val: &str) -> Self {
        self.default = val.into();
        self
    }
}
impl From<SpareString> for SpareParam {
    fn from(v: SpareString) -> Self {
        SpareParam::String {
            name: v.name,
            label: v.label,
            default: v.default,
        }
    }
}

pub struct SpareToggle {
    name: String,
    label: String,
    default: bool,
}
impl SpareToggle {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            default: false,
        }
    }
    pub fn with_default(mut self, val: bool) -> Self {
        self.default = val;
        self
    }
}
impl From<SpareToggle> for SpareParam {
    fn from(v: SpareToggle) -> Self {
        SpareParam::Toggle {
            name: v.name,
            label: v.label,
            default: v.default,
        }
    }
}

pub struct SpareColor {
    name: String,
    label: String,
    default: [f32; 3],
}
impl SpareColor {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            default: [1.0, 1.0, 1.0],
        }
    }
    pub fn with_default(mut self, val: [f32; 3]) -> Self {
        self.default = val;
        self
    }
}
impl From<SpareColor> for SpareParam {
    fn from(v: SpareColor) -> Self {
        SpareParam::Color {
            name: v.name,
            label: v.label,
            default: v.default,
        }
    }
}

pub struct SpareButton {
    name: String,
    label: String,
}
impl SpareButton {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
        }
    }
}
impl From<SpareButton> for SpareParam {
    fn from(v: SpareButton) -> Self {
        SpareParam::Button {
            name: v.name,
            label: v.label,
        }
    }
}

pub struct SpareMenu {
    name: String,
    label: String,
    default: i32,
    menu_items: Vec<(String, String)>,
}
impl SpareMenu {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            default: 0,
            menu_items: vec![],
        }
    }
    pub fn with_default(mut self, val: i32) -> Self {
        self.default = val;
        self
    }
    pub fn add_item(mut self, key: &str, label: &str) -> Self {
        self.menu_items.push((key.into(), label.into()));
        self
    }
}
impl From<SpareMenu> for SpareParam {
    fn from(v: SpareMenu) -> Self {
        assert!(
            !v.menu_items.is_empty(),
            "SpareMenu '{}' menu_items cannot be empty",
            v.name
        );
        assert!(
            v.default >= 0 && (v.default as usize) < v.menu_items.len(),
            "SpareMenu '{}' default_value {} is out of range [0, {})",
            v.name,
            v.default,
            v.menu_items.len()
        );
        SpareParam::Menu {
            name: v.name,
            label: v.label,
            default: v.default,
            menu_items: v.menu_items,
        }
    }
}

pub struct SpareFile {
    name: String,
    label: String,
    default: String,
}
impl SpareFile {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            default: String::new(),
        }
    }
    pub fn with_default(mut self, val: &str) -> Self {
        self.default = val.into();
        self
    }
}
impl From<SpareFile> for SpareParam {
    fn from(v: SpareFile) -> Self {
        SpareParam::File {
            name: v.name,
            label: v.label,
            default: v.default,
        }
    }
}

pub struct SpareNodePath {
    name: String,
    label: String,
    default: String,
}
impl SpareNodePath {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            default: String::new(),
        }
    }
    pub fn with_default(mut self, val: &str) -> Self {
        self.default = val.into();
        self
    }
}
impl From<SpareNodePath> for SpareParam {
    fn from(v: SpareNodePath) -> Self {
        SpareParam::NodePath {
            name: v.name,
            label: v.label,
            default: v.default,
        }
    }
}

pub struct SpareRampFloat {
    name: String,
    label: String,
}
impl SpareRampFloat {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
        }
    }
}
impl From<SpareRampFloat> for SpareParam {
    fn from(v: SpareRampFloat) -> Self {
        SpareParam::RampFloat {
            name: v.name,
            label: v.label,
        }
    }
}

pub struct SpareRampColor {
    name: String,
    label: String,
}
impl SpareRampColor {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
        }
    }
}
impl From<SpareRampColor> for SpareParam {
    fn from(v: SpareRampColor) -> Self {
        SpareParam::RampColor {
            name: v.name,
            label: v.label,
        }
    }
}

pub trait HoudiniNode {
    fn get_id(&self) -> usize;
    fn get_name(&self) -> &str;
    fn get_node_type(&self) -> &'static str;
    fn get_inputs(&self) -> &BTreeMap<usize, (usize, OutputPort)>;
    fn get_params(&self) -> &HashMap<String, ParamValue>;
    fn get_spare_params(&self) -> &[SpareParam] {
        &[]
    }
    fn get_dive_target(&self) -> Option<&'static str> {
        None
    }
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
            .map(|p| p.interpolation.as_hou_str().to_string())
            .collect();

        let keys: Vec<String> = points
            .iter()
            .map(|p| format!("{:.4}", p.position))
            .collect();

        let values: Vec<String> = points
            .iter()
            .map(|p| {
                if is_color {
                    assert!(
                        p.value.len() >= 3,
                        "Color Ramp point must have at least 3 elements."
                    );
                    format!("({:.4}, {:.4}, {:.4})", p.value[0], p.value[1], p.value[2])
                } else {
                    assert!(
                        !p.value.is_empty(),
                        "Float Ramp point must have at least 1 element."
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
    #[should_panic(expected = "Color Ramp point must have at least 3 elements.")]
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
    #[should_panic(expected = "Float Ramp point must have at least 1 element.")]
    fn test_ramp_serialization_empty_value_panics() {
        let invalid_empty_val_ramp = ParamValue::Ramp(vec![RampPoint {
            position: 0.0,
            value: vec![],
            interpolation: RampInterpolation::Linear,
        }]);

        let _ = invalid_empty_val_ramp.to_python_expr();
    }
}
