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
