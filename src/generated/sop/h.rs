#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHaircardgenWidthmethod {
    SpecifyWidth = 0,
    ComputeUsingClusterDistribution = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHaircardgenLengthdivmethod {
    UniformSegmentCount = 0,
    SegmentLength = 1,
    Refine = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHaircardgenUvmethod {
    Regular = 0,
    UvReferenceFile = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHaircardgenUvregpackingmethod {
    SingleTile = 0,
    FixedScale = 1,
}

#[derive(Debug, Clone)]
pub struct SopHaircardgen {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHaircardgen {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Hair Curves"
    pub fn set_input_hair_curves<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Hair Curves" and specifies the output index of the target node.
    pub fn set_input_hair_curves_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Cluster Points"
    pub fn set_input_cluster_points<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Cluster Points" and specifies the output index of the target node.
    pub fn set_input_cluster_points_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Normal Override"
    pub fn set_input_normal_override<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Normal Override" and specifies the output index of the target node.
    pub fn set_input_normal_override_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_mincurvelength(mut self, val: f32) -> Self {
        self.params.insert("mincurvelength".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_mincurvelength_expr(mut self, expr: &str) -> Self {
        self.params.insert("mincurvelength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clustercurveu(mut self, val: f32) -> Self {
        self.params.insert("clustercurveu".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_clustercurveu_expr(mut self, expr: &str) -> Self {
        self.params.insert("clustercurveu".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_snaprootsblenddist(mut self, val: f32) -> Self {
        self.params.insert("snaprootsblenddist".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_snaprootsblenddist_expr(mut self, expr: &str) -> Self {
        self.params.insert("snaprootsblenddist".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_snaprootsblendbias(mut self, val: f32) -> Self {
        self.params.insert("snaprootsblendbias".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_snaprootsblendbias_expr(mut self, expr: &str) -> Self {
        self.params.insert("snaprootsblendbias".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_width(mut self, val: f32) -> Self {
        self.params.insert("width".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_width_expr(mut self, expr: &str) -> Self {
        self.params.insert("width".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_widthscale(mut self, val: f32) -> Self {
        self.params.insert("widthscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_widthscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("widthscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_widthmax(mut self, val: f32) -> Self {
        self.params.insert("widthmax".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_widthmax_expr(mut self, expr: &str) -> Self {
        self.params.insert("widthmax".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_extralength(mut self, val: f32) -> Self {
        self.params.insert("extralength".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_extralength_expr(mut self, expr: &str) -> Self {
        self.params.insert("extralength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lengthseg(mut self, val: f32) -> Self {
        self.params.insert("lengthseg".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_lengthseg_expr(mut self, expr: &str) -> Self {
        self.params.insert("lengthseg".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lengthrefinedivs(mut self, val: f32) -> Self {
        self.params.insert("lengthrefinedivs".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_lengthrefinedivs_expr(mut self, expr: &str) -> Self {
        self.params.insert("lengthrefinedivs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_bendrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("bendrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_bendrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("bendrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_hasclusterpoints(mut self, val: i32) -> Self {
        self.params.insert("hasclusterpoints".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_hasclusterpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert("hasclusterpoints".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hasnormaloverride(mut self, val: i32) -> Self {
        self.params.insert("hasnormaloverride".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_hasnormaloverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("hasnormaloverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_numcards(mut self, val: i32) -> Self {
        self.params.insert("numcards".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_numcards_expr(mut self, expr: &str) -> Self {
        self.params.insert("numcards".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clusterseed(mut self, val: i32) -> Self {
        self.params.insert("clusterseed".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_clusterseed_expr(mut self, expr: &str) -> Self {
        self.params.insert("clusterseed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_orientsmoothiters(mut self, val: i32) -> Self {
        self.params.insert("orientsmoothiters".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_orientsmoothiters_expr(mut self, expr: &str) -> Self {
        self.params.insert("orientsmoothiters".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_widthdivs(mut self, val: i32) -> Self {
        self.params.insert("widthdivs".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_widthdivs_expr(mut self, expr: &str) -> Self {
        self.params.insert("widthdivs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lengthuniformdivs(mut self, val: i32) -> Self {
        self.params.insert("lengthuniformdivs".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_lengthuniformdivs_expr(mut self, expr: &str) -> Self {
        self.params.insert("lengthuniformdivs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvregpadding(mut self, val: i32) -> Self {
        self.params.insert("uvregpadding".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_uvregpadding_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvregpadding".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_widthmethod(mut self, val: SopHaircardgenWidthmethod) -> Self {
        self.params.insert("widthmethod".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_widthmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert("widthmethod".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lengthdivmethod(mut self, val: SopHaircardgenLengthdivmethod) -> Self {
        self.params.insert("lengthdivmethod".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_lengthdivmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert("lengthdivmethod".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvmethod(mut self, val: SopHaircardgenUvmethod) -> Self {
        self.params.insert("uvmethod".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uvmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvmethod".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvregpackingmethod(mut self, val: SopHaircardgenUvregpackingmethod) -> Self {
        self.params.insert("uvregpackingmethod".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uvregpackingmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvregpackingmethod".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_widthramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("widthramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_widthramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("widthramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvreffile(mut self, val: &str) -> Self {
        self.params.insert("uvreffile".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uvreffile_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvreffile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvrefchannel(mut self, val: &str) -> Self {
        self.params.insert("uvrefchannel".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uvrefchannel_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvrefchannel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_ignoreshortcurves(mut self, val: bool) -> Self {
        self.params.insert("ignoreshortcurves".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_ignoreshortcurves_expr(mut self, expr: &str) -> Self {
        self.params.insert("ignoreshortcurves".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usecustomclusterpoints(mut self, val: bool) -> Self {
        self.params.insert("usecustomclusterpoints".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usecustomclusterpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert("usecustomclusterpoints".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_orientsmooth(mut self, val: bool) -> Self {
        self.params.insert("orientsmooth".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_orientsmooth_expr(mut self, expr: &str) -> Self {
        self.params.insert("orientsmooth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_snaproots(mut self, val: bool) -> Self {
        self.params.insert("snaproots".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_snaproots_expr(mut self, expr: &str) -> Self {
        self.params.insert("snaproots".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvenable(mut self, val: bool) -> Self {
        self.params.insert("uvenable".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_uvenable_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvenable".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHaircardgen {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "haircardgen"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpBlendoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpBlendoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpClumpsizeoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpClumpsizeoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpMethod {
    LinearBlend = 0,
    ExtrudeAndBlend = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpTightnessoverride {
    NoOverride = 0,
    ClumpAttribute = 1,
    GuideAttribute = 2,
    SkinAttribute = 3,
    Texture = 4,
    TexturePrimitive = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpTightnessoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpStrayamountoverride {
    NoOverride = 0,
    ClumpAttribute = 1,
    GuideAttribute = 2,
    SkinAttribute = 3,
    Texture = 4,
    TexturePrimitive = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpStrayamountoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpStrayrateoverride {
    NoOverride = 0,
    ClumpAttribute = 1,
    GuideAttribute = 2,
    SkinAttribute = 3,
    Texture = 4,
    TexturePrimitive = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpStrayrateoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpStrayfalloffoverride {
    NoOverride = 0,
    ClumpAttribute = 1,
    GuideAttribute = 2,
    SkinAttribute = 3,
    Texture = 4,
    TexturePrimitive = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpStrayfalloffoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpIterationsoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpIterationsoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpSizereductionoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpSizereductionoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpGoalfeedbackoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpGoalfeedbackoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpTightnessreductionoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpTightnessreductionoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpCurlampoverride {
    NoOverride = 0,
    ClumpAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpCurlampoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpCurlfreqoverride {
    NoOverride = 0,
    ClumpAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpCurlfreqoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone)]
pub struct SopHairclump {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHairclump {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Guides"
    pub fn set_input_guides<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Guides" and specifies the output index of the target node.
    pub fn set_input_guides_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Skin VDB"
    pub fn set_input_skin_vdb<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Skin VDB" and specifies the output index of the target node.
    pub fn set_input_skin_vdb_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Custom Clump Curves"
    pub fn set_input_custom_clump_curves<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Custom Clump Curves" and specifies the output index of the target node.
    pub fn set_input_custom_clump_curves_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsize(mut self, val: f32) -> Self {
        self.params.insert("clumpsize".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_clumpsize_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_crossoverrate(mut self, val: f32) -> Self {
        self.params.insert("crossoverrate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_crossoverrate_expr(mut self, expr: &str) -> Self {
        self.params.insert("crossoverrate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_extendtomatch(mut self, val: f32) -> Self {
        self.params.insert("extendtomatch".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_extendtomatch_expr(mut self, expr: &str) -> Self {
        self.params.insert("extendtomatch".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shortentomatch(mut self, val: f32) -> Self {
        self.params.insert("shortentomatch".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_shortentomatch_expr(mut self, expr: &str) -> Self {
        self.params.insert("shortentomatch".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hairwidth(mut self, val: f32) -> Self {
        self.params.insert("hairwidth".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_hairwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert("hairwidth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hairwidthscale(mut self, val: f32) -> Self {
        self.params.insert("hairwidthscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_hairwidthscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("hairwidthscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightness(mut self, val: f32) -> Self {
        self.params.insert("tightness".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tightness_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightness".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamount(mut self, val: f32) -> Self {
        self.params.insert("strayamount".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_strayamount_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamount".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayrate(mut self, val: f32) -> Self {
        self.params.insert("strayrate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_strayrate_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayrate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloff(mut self, val: f32) -> Self {
        self.params.insert("strayfalloff".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_strayfalloff_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloff".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterations(mut self, val: f32) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereduction(mut self, val: f32) -> Self {
        self.params.insert("sizereduction".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_sizereduction_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereduction".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedback(mut self, val: f32) -> Self {
        self.params.insert("goalfeedback".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_goalfeedback_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedback".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreduction(mut self, val: f32) -> Self {
        self.params.insert("tightnessreduction".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tightnessreduction_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreduction".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlamp(mut self, val: f32) -> Self {
        self.params.insert("curlamp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_curlamp_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlamp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreq(mut self, val: f32) -> Self {
        self.params.insert("curlfreq".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_curlfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreq".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_blendinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("blendinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_blendinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blendoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("blendoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_blendoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsizeinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("clumpsizeinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_clumpsizeinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsizeinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsizeoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("clumpsizeoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_clumpsizeoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsizeoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tightnessinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tightnessinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tightnessoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tightnessoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamountinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("strayamountinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_strayamountinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamountinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamountoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("strayamountoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_strayamountoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamountoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayrateinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("strayrateinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_strayrateinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayrateinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayrateoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("strayrateoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_strayrateoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayrateoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloffinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("strayfalloffinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_strayfalloffinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloffinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloffoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("strayfalloffoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_strayfalloffoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloffoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationsinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("iterationsinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_iterationsinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationsinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationsoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("iterationsoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_iterationsoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationsoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereductioninrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("sizereductioninrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_sizereductioninrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereductioninrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereductionoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("sizereductionoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_sizereductionoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereductionoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedbackinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("goalfeedbackinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_goalfeedbackinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedbackinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedbackoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("goalfeedbackoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_goalfeedbackoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedbackoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductioninrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tightnessreductioninrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tightnessreductioninrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductioninrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductionoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tightnessreductionoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tightnessreductionoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductionoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlampinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("curlampinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_curlampinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlampinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlampoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("curlampoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_curlampoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlampoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("curlfreqinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_curlfreqinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("curlfreqoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_curlfreqoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_blendoverride(mut self, val: SopHairclumpBlendoverride) -> Self {
        self.params.insert("blendoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_blendoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blendoptions(mut self, val: SopHairclumpBlendoptions) -> Self {
        self.params.insert("blendoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_blendoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsizeoverride(mut self, val: SopHairclumpClumpsizeoverride) -> Self {
        self.params.insert("clumpsizeoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_clumpsizeoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsizeoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsizeoptions(mut self, val: SopHairclumpClumpsizeoptions) -> Self {
        self.params.insert("clumpsizeoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_clumpsizeoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsizeoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_method(mut self, val: SopHairclumpMethod) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessoverride(mut self, val: SopHairclumpTightnessoverride) -> Self {
        self.params.insert("tightnessoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tightnessoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessoptions(mut self, val: SopHairclumpTightnessoptions) -> Self {
        self.params.insert("tightnessoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tightnessoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamountoverride(mut self, val: SopHairclumpStrayamountoverride) -> Self {
        self.params.insert("strayamountoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_strayamountoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamountoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamountoptions(mut self, val: SopHairclumpStrayamountoptions) -> Self {
        self.params.insert("strayamountoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_strayamountoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamountoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayrateoverride(mut self, val: SopHairclumpStrayrateoverride) -> Self {
        self.params.insert("strayrateoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_strayrateoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayrateoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayrateoptions(mut self, val: SopHairclumpStrayrateoptions) -> Self {
        self.params.insert("strayrateoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_strayrateoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayrateoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloffoverride(mut self, val: SopHairclumpStrayfalloffoverride) -> Self {
        self.params.insert("strayfalloffoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_strayfalloffoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloffoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloffoptions(mut self, val: SopHairclumpStrayfalloffoptions) -> Self {
        self.params.insert("strayfalloffoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_strayfalloffoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloffoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationsoverride(mut self, val: SopHairclumpIterationsoverride) -> Self {
        self.params.insert("iterationsoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_iterationsoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationsoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationsoptions(mut self, val: SopHairclumpIterationsoptions) -> Self {
        self.params.insert("iterationsoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_iterationsoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationsoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereductionoverride(mut self, val: SopHairclumpSizereductionoverride) -> Self {
        self.params.insert("sizereductionoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_sizereductionoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereductionoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereductionoptions(mut self, val: SopHairclumpSizereductionoptions) -> Self {
        self.params.insert("sizereductionoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_sizereductionoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereductionoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedbackoverride(mut self, val: SopHairclumpGoalfeedbackoverride) -> Self {
        self.params.insert("goalfeedbackoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_goalfeedbackoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedbackoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedbackoptions(mut self, val: SopHairclumpGoalfeedbackoptions) -> Self {
        self.params.insert("goalfeedbackoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_goalfeedbackoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedbackoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductionoverride(mut self, val: SopHairclumpTightnessreductionoverride) -> Self {
        self.params.insert("tightnessreductionoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tightnessreductionoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductionoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductionoptions(mut self, val: SopHairclumpTightnessreductionoptions) -> Self {
        self.params.insert("tightnessreductionoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tightnessreductionoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductionoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlampoverride(mut self, val: SopHairclumpCurlampoverride) -> Self {
        self.params.insert("curlampoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_curlampoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlampoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlampoptions(mut self, val: SopHairclumpCurlampoptions) -> Self {
        self.params.insert("curlampoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_curlampoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlampoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqoverride(mut self, val: SopHairclumpCurlfreqoverride) -> Self {
        self.params.insert("curlfreqoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_curlfreqoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqoptions(mut self, val: SopHairclumpCurlfreqoptions) -> Self {
        self.params.insert("curlfreqoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_curlfreqoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_blendremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("blendremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_blendremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsizeremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("clumpsizeremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_clumpsizeremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsizeremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("tightnessremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_tightnessremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamountremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("strayamountremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_strayamountremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamountremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayrateremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("strayrateremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_strayrateremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayrateremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloffremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("strayfalloffremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_strayfalloffremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloffremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpprofile(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("clumpprofile".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_clumpprofile_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpprofile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationsremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("iterationsremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_iterationsremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationsremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereductionremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("sizereductionremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_sizereductionremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereductionremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedbackremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("goalfeedbackremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_goalfeedbackremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedbackremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductionremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("tightnessreductionremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_tightnessreductionremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductionremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlampremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("curlampremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_curlampremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlampremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlampramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("curlampramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_curlampramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlampramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("curlfreqremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_curlfreqremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("curlfreqramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_curlfreqramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_blendcurveattrib(mut self, val: &str) -> Self {
        self.params.insert("blendcurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_blendcurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendcurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blendattrib(mut self, val: &str) -> Self {
        self.params.insert("blendattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_blendattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blendtexture(mut self, val: &str) -> Self {
        self.params.insert("blendtexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_blendtexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendtexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blendtextureprim(mut self, val: &str) -> Self {
        self.params.insert("blendtextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_blendtextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendtextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_skinmaskattrib(mut self, val: &str) -> Self {
        self.params.insert("skinmaskattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_skinmaskattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("skinmaskattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsizeattrib(mut self, val: &str) -> Self {
        self.params.insert("clumpsizeattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpsizeattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsizeattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsizetexture(mut self, val: &str) -> Self {
        self.params.insert("clumpsizetexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpsizetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsizetexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsizetextureprim(mut self, val: &str) -> Self {
        self.params.insert("clumpsizetextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpsizetextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsizetextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpidinputattrib(mut self, val: &str) -> Self {
        self.params.insert("clumpidinputattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpidinputattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpidinputattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_orientinputattrib(mut self, val: &str) -> Self {
        self.params.insert("orientinputattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_orientinputattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("orientinputattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnesscurveattrib(mut self, val: &str) -> Self {
        self.params.insert("tightnesscurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnesscurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnesscurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessclumpattrib(mut self, val: &str) -> Self {
        self.params.insert("tightnessclumpattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnessclumpattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessclumpattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessattrib(mut self, val: &str) -> Self {
        self.params.insert("tightnessattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnessattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnesstexture(mut self, val: &str) -> Self {
        self.params.insert("tightnesstexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnesstexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnesstexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnesstextureprim(mut self, val: &str) -> Self {
        self.params.insert("tightnesstextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnesstextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnesstextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamountcurveattrib(mut self, val: &str) -> Self {
        self.params.insert("strayamountcurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayamountcurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamountcurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamountclumpattrib(mut self, val: &str) -> Self {
        self.params.insert("strayamountclumpattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayamountclumpattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamountclumpattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamountattrib(mut self, val: &str) -> Self {
        self.params.insert("strayamountattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayamountattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamountattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamounttexture(mut self, val: &str) -> Self {
        self.params.insert("strayamounttexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayamounttexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamounttexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamounttextureprim(mut self, val: &str) -> Self {
        self.params.insert("strayamounttextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayamounttextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamounttextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayratecurveattrib(mut self, val: &str) -> Self {
        self.params.insert("strayratecurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayratecurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayratecurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayrateclumpattrib(mut self, val: &str) -> Self {
        self.params.insert("strayrateclumpattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayrateclumpattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayrateclumpattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayrateattrib(mut self, val: &str) -> Self {
        self.params.insert("strayrateattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayrateattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayrateattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayratetexture(mut self, val: &str) -> Self {
        self.params.insert("strayratetexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayratetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayratetexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayratetextureprim(mut self, val: &str) -> Self {
        self.params.insert("strayratetextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayratetextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayratetextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloffcurveattrib(mut self, val: &str) -> Self {
        self.params.insert("strayfalloffcurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayfalloffcurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloffcurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloffclumpattrib(mut self, val: &str) -> Self {
        self.params.insert("strayfalloffclumpattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayfalloffclumpattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloffclumpattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloffattrib(mut self, val: &str) -> Self {
        self.params.insert("strayfalloffattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayfalloffattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloffattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfallofftexture(mut self, val: &str) -> Self {
        self.params.insert("strayfallofftexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayfallofftexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfallofftexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfallofftextureprim(mut self, val: &str) -> Self {
        self.params.insert("strayfallofftextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayfallofftextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfallofftextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationscurveattrib(mut self, val: &str) -> Self {
        self.params.insert("iterationscurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_iterationscurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationscurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationsattrib(mut self, val: &str) -> Self {
        self.params.insert("iterationsattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_iterationsattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationsattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationstexture(mut self, val: &str) -> Self {
        self.params.insert("iterationstexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_iterationstexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationstexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationstextureprim(mut self, val: &str) -> Self {
        self.params.insert("iterationstextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_iterationstextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationstextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereductionattrib(mut self, val: &str) -> Self {
        self.params.insert("sizereductionattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sizereductionattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereductionattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereductiontexture(mut self, val: &str) -> Self {
        self.params.insert("sizereductiontexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sizereductiontexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereductiontexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereductiontextureprim(mut self, val: &str) -> Self {
        self.params.insert("sizereductiontextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sizereductiontextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereductiontextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedbackattrib(mut self, val: &str) -> Self {
        self.params.insert("goalfeedbackattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_goalfeedbackattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedbackattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedbacktexture(mut self, val: &str) -> Self {
        self.params.insert("goalfeedbacktexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_goalfeedbacktexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedbacktexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedbacktextureprim(mut self, val: &str) -> Self {
        self.params.insert("goalfeedbacktextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_goalfeedbacktextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedbacktextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductioncurveattrib(mut self, val: &str) -> Self {
        self.params.insert("tightnessreductioncurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnessreductioncurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductioncurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductionattrib(mut self, val: &str) -> Self {
        self.params.insert("tightnessreductionattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnessreductionattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductionattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductiontexture(mut self, val: &str) -> Self {
        self.params.insert("tightnessreductiontexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnessreductiontexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductiontexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductiontextureprim(mut self, val: &str) -> Self {
        self.params.insert("tightnessreductiontextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnessreductiontextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductiontextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlampcurveattrib(mut self, val: &str) -> Self {
        self.params.insert("curlampcurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_curlampcurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlampcurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlampattrib(mut self, val: &str) -> Self {
        self.params.insert("curlampattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_curlampattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlampattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlamptexture(mut self, val: &str) -> Self {
        self.params.insert("curlamptexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_curlamptexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlamptexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlamptextureprim(mut self, val: &str) -> Self {
        self.params.insert("curlamptextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_curlamptextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlamptextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqcurveattrib(mut self, val: &str) -> Self {
        self.params.insert("curlfreqcurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_curlfreqcurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqcurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqattrib(mut self, val: &str) -> Self {
        self.params.insert("curlfreqattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_curlfreqattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqtexture(mut self, val: &str) -> Self {
        self.params.insert("curlfreqtexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_curlfreqtexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqtexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqtextureprim(mut self, val: &str) -> Self {
        self.params.insert("curlfreqtextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_curlfreqtextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqtextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpidoutputattrib(mut self, val: &str) -> Self {
        self.params.insert("clumpidoutputattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpidoutputattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpidoutputattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessoutputattrib(mut self, val: &str) -> Self {
        self.params.insert("tightnessoutputattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnessoutputattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessoutputattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumppointattribs(mut self, val: &str) -> Self {
        self.params.insert("clumppointattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumppointattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumppointattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpprimattribs(mut self, val: &str) -> Self {
        self.params.insert("clumpprimattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpprimattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpprimattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_legacymasking(mut self, val: bool) -> Self {
        self.params.insert("legacymasking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_legacymasking_expr(mut self, expr: &str) -> Self {
        self.params.insert("legacymasking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useskinmask(mut self, val: bool) -> Self {
        self.params.insert("useskinmask".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useskinmask_expr(mut self, expr: &str) -> Self {
        self.params.insert("useskinmask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_searchbeyondradius(mut self, val: bool) -> Self {
        self.params.insert("searchbeyondradius".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_searchbeyondradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("searchbeyondradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpwithinclumps(mut self, val: bool) -> Self {
        self.params.insert("clumpwithinclumps".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_clumpwithinclumps_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpwithinclumps".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useorientinputattrib(mut self, val: bool) -> Self {
        self.params.insert("useorientinputattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useorientinputattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("useorientinputattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_preservelength(mut self, val: bool) -> Self {
        self.params.insert("preservelength".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_preservelength_expr(mut self, expr: &str) -> Self {
        self.params.insert("preservelength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_accuratebundling(mut self, val: bool) -> Self {
        self.params.insert("accuratebundling".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_accuratebundling_expr(mut self, expr: &str) -> Self {
        self.params.insert("accuratebundling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enablecurling(mut self, val: bool) -> Self {
        self.params.insert("enablecurling".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enablecurling_expr(mut self, expr: &str) -> Self {
        self.params.insert("enablecurling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_createclumpidattrib(mut self, val: bool) -> Self {
        self.params.insert("createclumpidattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_createclumpidattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("createclumpidattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_createtightnessattrib(mut self, val: bool) -> Self {
        self.params.insert("createtightnessattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_createtightnessattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("createtightnessattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHairclump {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "hairclump"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreBlendoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreBlendoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreClumpsizeoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreClumpsizeoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreMethod {
    LinearBlend = 0,
    ExtrudeAndBlend = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreTightnessoverride {
    NoOverride = 0,
    ClumpAttribute = 1,
    GuideAttribute = 2,
    SkinAttribute = 3,
    Texture = 4,
    TexturePrimitive = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreTightnessoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreStrayamountoverride {
    NoOverride = 0,
    ClumpAttribute = 1,
    GuideAttribute = 2,
    SkinAttribute = 3,
    Texture = 4,
    TexturePrimitive = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreStrayamountoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreStrayrateoverride {
    NoOverride = 0,
    ClumpAttribute = 1,
    GuideAttribute = 2,
    SkinAttribute = 3,
    Texture = 4,
    TexturePrimitive = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreStrayrateoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreStrayfalloffoverride {
    NoOverride = 0,
    ClumpAttribute = 1,
    GuideAttribute = 2,
    SkinAttribute = 3,
    Texture = 4,
    TexturePrimitive = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreStrayfalloffoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreIterationsoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreIterationsoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreSizereductionoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreSizereductionoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreGoalfeedbackoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreGoalfeedbackoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreTightnessreductionoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreTightnessreductionoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreCurlampoverride {
    NoOverride = 0,
    ClumpAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreCurlampoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreCurlfreqoverride {
    NoOverride = 0,
    ClumpAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairclumpcoreCurlfreqoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone)]
pub struct SopHairclumpcore {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHairclumpcore {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Guides"
    pub fn set_input_guides<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Guides" and specifies the output index of the target node.
    pub fn set_input_guides_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Volumes"
    pub fn set_input_volumes<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Volumes" and specifies the output index of the target node.
    pub fn set_input_volumes_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Custom Clump Guides"
    pub fn set_input_custom_clump_guides<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Custom Clump Guides" and specifies the output index of the target node.
    pub fn set_input_custom_clump_guides_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsize(mut self, val: f32) -> Self {
        self.params.insert("clumpsize".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_clumpsize_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_crossoverrate(mut self, val: f32) -> Self {
        self.params.insert("crossoverrate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_crossoverrate_expr(mut self, expr: &str) -> Self {
        self.params.insert("crossoverrate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_extendtomatch(mut self, val: f32) -> Self {
        self.params.insert("extendtomatch".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_extendtomatch_expr(mut self, expr: &str) -> Self {
        self.params.insert("extendtomatch".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shortentomatch(mut self, val: f32) -> Self {
        self.params.insert("shortentomatch".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_shortentomatch_expr(mut self, expr: &str) -> Self {
        self.params.insert("shortentomatch".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hairwidth(mut self, val: f32) -> Self {
        self.params.insert("hairwidth".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_hairwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert("hairwidth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hairwidthscale(mut self, val: f32) -> Self {
        self.params.insert("hairwidthscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_hairwidthscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("hairwidthscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightness(mut self, val: f32) -> Self {
        self.params.insert("tightness".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tightness_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightness".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamount(mut self, val: f32) -> Self {
        self.params.insert("strayamount".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_strayamount_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamount".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayrate(mut self, val: f32) -> Self {
        self.params.insert("strayrate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_strayrate_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayrate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloff(mut self, val: f32) -> Self {
        self.params.insert("strayfalloff".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_strayfalloff_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloff".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterations(mut self, val: f32) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereduction(mut self, val: f32) -> Self {
        self.params.insert("sizereduction".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_sizereduction_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereduction".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedback(mut self, val: f32) -> Self {
        self.params.insert("goalfeedback".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_goalfeedback_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedback".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreduction(mut self, val: f32) -> Self {
        self.params.insert("tightnessreduction".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tightnessreduction_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreduction".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlamp(mut self, val: f32) -> Self {
        self.params.insert("curlamp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_curlamp_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlamp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreq(mut self, val: f32) -> Self {
        self.params.insert("curlfreq".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_curlfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreq".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_blendinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("blendinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_blendinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blendoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("blendoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_blendoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsizeinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("clumpsizeinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_clumpsizeinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsizeinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsizeoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("clumpsizeoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_clumpsizeoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsizeoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tightnessinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tightnessinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tightnessoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tightnessoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamountinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("strayamountinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_strayamountinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamountinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamountoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("strayamountoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_strayamountoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamountoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayrateinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("strayrateinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_strayrateinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayrateinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayrateoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("strayrateoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_strayrateoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayrateoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloffinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("strayfalloffinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_strayfalloffinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloffinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloffoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("strayfalloffoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_strayfalloffoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloffoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationsinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("iterationsinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_iterationsinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationsinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationsoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("iterationsoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_iterationsoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationsoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereductioninrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("sizereductioninrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_sizereductioninrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereductioninrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereductionoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("sizereductionoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_sizereductionoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereductionoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedbackinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("goalfeedbackinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_goalfeedbackinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedbackinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedbackoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("goalfeedbackoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_goalfeedbackoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedbackoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductioninrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tightnessreductioninrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tightnessreductioninrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductioninrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductionoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tightnessreductionoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tightnessreductionoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductionoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlampinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("curlampinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_curlampinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlampinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlampoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("curlampoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_curlampoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlampoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("curlfreqinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_curlfreqinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("curlfreqoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_curlfreqoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_blendoverride(mut self, val: SopHairclumpcoreBlendoverride) -> Self {
        self.params.insert("blendoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_blendoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blendoptions(mut self, val: SopHairclumpcoreBlendoptions) -> Self {
        self.params.insert("blendoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_blendoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsizeoverride(mut self, val: SopHairclumpcoreClumpsizeoverride) -> Self {
        self.params.insert("clumpsizeoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_clumpsizeoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsizeoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsizeoptions(mut self, val: SopHairclumpcoreClumpsizeoptions) -> Self {
        self.params.insert("clumpsizeoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_clumpsizeoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsizeoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_method(mut self, val: SopHairclumpcoreMethod) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessoverride(mut self, val: SopHairclumpcoreTightnessoverride) -> Self {
        self.params.insert("tightnessoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tightnessoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessoptions(mut self, val: SopHairclumpcoreTightnessoptions) -> Self {
        self.params.insert("tightnessoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tightnessoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamountoverride(mut self, val: SopHairclumpcoreStrayamountoverride) -> Self {
        self.params.insert("strayamountoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_strayamountoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamountoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamountoptions(mut self, val: SopHairclumpcoreStrayamountoptions) -> Self {
        self.params.insert("strayamountoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_strayamountoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamountoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayrateoverride(mut self, val: SopHairclumpcoreStrayrateoverride) -> Self {
        self.params.insert("strayrateoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_strayrateoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayrateoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayrateoptions(mut self, val: SopHairclumpcoreStrayrateoptions) -> Self {
        self.params.insert("strayrateoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_strayrateoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayrateoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloffoverride(mut self, val: SopHairclumpcoreStrayfalloffoverride) -> Self {
        self.params.insert("strayfalloffoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_strayfalloffoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloffoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloffoptions(mut self, val: SopHairclumpcoreStrayfalloffoptions) -> Self {
        self.params.insert("strayfalloffoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_strayfalloffoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloffoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationsoverride(mut self, val: SopHairclumpcoreIterationsoverride) -> Self {
        self.params.insert("iterationsoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_iterationsoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationsoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationsoptions(mut self, val: SopHairclumpcoreIterationsoptions) -> Self {
        self.params.insert("iterationsoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_iterationsoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationsoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereductionoverride(mut self, val: SopHairclumpcoreSizereductionoverride) -> Self {
        self.params.insert("sizereductionoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_sizereductionoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereductionoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereductionoptions(mut self, val: SopHairclumpcoreSizereductionoptions) -> Self {
        self.params.insert("sizereductionoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_sizereductionoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereductionoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedbackoverride(mut self, val: SopHairclumpcoreGoalfeedbackoverride) -> Self {
        self.params.insert("goalfeedbackoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_goalfeedbackoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedbackoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedbackoptions(mut self, val: SopHairclumpcoreGoalfeedbackoptions) -> Self {
        self.params.insert("goalfeedbackoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_goalfeedbackoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedbackoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductionoverride(mut self, val: SopHairclumpcoreTightnessreductionoverride) -> Self {
        self.params.insert("tightnessreductionoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tightnessreductionoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductionoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductionoptions(mut self, val: SopHairclumpcoreTightnessreductionoptions) -> Self {
        self.params.insert("tightnessreductionoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tightnessreductionoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductionoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlampoverride(mut self, val: SopHairclumpcoreCurlampoverride) -> Self {
        self.params.insert("curlampoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_curlampoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlampoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlampoptions(mut self, val: SopHairclumpcoreCurlampoptions) -> Self {
        self.params.insert("curlampoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_curlampoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlampoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqoverride(mut self, val: SopHairclumpcoreCurlfreqoverride) -> Self {
        self.params.insert("curlfreqoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_curlfreqoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqoptions(mut self, val: SopHairclumpcoreCurlfreqoptions) -> Self {
        self.params.insert("curlfreqoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_curlfreqoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_blendremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("blendremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_blendremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsizeremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("clumpsizeremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_clumpsizeremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsizeremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("tightnessremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_tightnessremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamountremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("strayamountremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_strayamountremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamountremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayrateremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("strayrateremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_strayrateremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayrateremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloffremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("strayfalloffremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_strayfalloffremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloffremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpprofile(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("clumpprofile".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_clumpprofile_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpprofile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationsremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("iterationsremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_iterationsremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationsremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereductionremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("sizereductionremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_sizereductionremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereductionremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedbackremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("goalfeedbackremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_goalfeedbackremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedbackremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductionremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("tightnessreductionremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_tightnessreductionremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductionremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlampremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("curlampremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_curlampremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlampremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlampramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("curlampramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_curlampramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlampramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("curlfreqremapramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_curlfreqremapramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqremapramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("curlfreqramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_curlfreqramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_uvattrib(mut self, val: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uvattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blendcurveattrib(mut self, val: &str) -> Self {
        self.params.insert("blendcurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_blendcurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendcurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blendattrib(mut self, val: &str) -> Self {
        self.params.insert("blendattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_blendattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blendtexture(mut self, val: &str) -> Self {
        self.params.insert("blendtexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_blendtexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendtexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blendtextureprim(mut self, val: &str) -> Self {
        self.params.insert("blendtextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_blendtextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("blendtextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_skinmaskattrib(mut self, val: &str) -> Self {
        self.params.insert("skinmaskattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_skinmaskattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("skinmaskattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsizeattrib(mut self, val: &str) -> Self {
        self.params.insert("clumpsizeattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpsizeattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsizeattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsizetexture(mut self, val: &str) -> Self {
        self.params.insert("clumpsizetexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpsizetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsizetexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpsizetextureprim(mut self, val: &str) -> Self {
        self.params.insert("clumpsizetextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpsizetextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpsizetextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnesscurveattrib(mut self, val: &str) -> Self {
        self.params.insert("tightnesscurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnesscurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnesscurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessclumpattrib(mut self, val: &str) -> Self {
        self.params.insert("tightnessclumpattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnessclumpattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessclumpattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessattrib(mut self, val: &str) -> Self {
        self.params.insert("tightnessattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnessattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnesstexture(mut self, val: &str) -> Self {
        self.params.insert("tightnesstexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnesstexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnesstexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnesstextureprim(mut self, val: &str) -> Self {
        self.params.insert("tightnesstextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnesstextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnesstextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamountcurveattrib(mut self, val: &str) -> Self {
        self.params.insert("strayamountcurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayamountcurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamountcurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamountclumpattrib(mut self, val: &str) -> Self {
        self.params.insert("strayamountclumpattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayamountclumpattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamountclumpattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamountattrib(mut self, val: &str) -> Self {
        self.params.insert("strayamountattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayamountattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamountattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamounttexture(mut self, val: &str) -> Self {
        self.params.insert("strayamounttexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayamounttexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamounttexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayamounttextureprim(mut self, val: &str) -> Self {
        self.params.insert("strayamounttextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayamounttextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayamounttextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayratecurveattrib(mut self, val: &str) -> Self {
        self.params.insert("strayratecurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayratecurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayratecurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayrateclumpattrib(mut self, val: &str) -> Self {
        self.params.insert("strayrateclumpattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayrateclumpattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayrateclumpattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayrateattrib(mut self, val: &str) -> Self {
        self.params.insert("strayrateattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayrateattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayrateattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayratetexture(mut self, val: &str) -> Self {
        self.params.insert("strayratetexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayratetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayratetexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayratetextureprim(mut self, val: &str) -> Self {
        self.params.insert("strayratetextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayratetextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayratetextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloffcurveattrib(mut self, val: &str) -> Self {
        self.params.insert("strayfalloffcurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayfalloffcurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloffcurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloffclumpattrib(mut self, val: &str) -> Self {
        self.params.insert("strayfalloffclumpattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayfalloffclumpattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloffclumpattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfalloffattrib(mut self, val: &str) -> Self {
        self.params.insert("strayfalloffattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayfalloffattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfalloffattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfallofftexture(mut self, val: &str) -> Self {
        self.params.insert("strayfallofftexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayfallofftexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfallofftexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strayfallofftextureprim(mut self, val: &str) -> Self {
        self.params.insert("strayfallofftextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strayfallofftextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("strayfallofftextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationscurveattrib(mut self, val: &str) -> Self {
        self.params.insert("iterationscurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_iterationscurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationscurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationsattrib(mut self, val: &str) -> Self {
        self.params.insert("iterationsattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_iterationsattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationsattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationstexture(mut self, val: &str) -> Self {
        self.params.insert("iterationstexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_iterationstexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationstexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterationstextureprim(mut self, val: &str) -> Self {
        self.params.insert("iterationstextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_iterationstextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationstextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereductionattrib(mut self, val: &str) -> Self {
        self.params.insert("sizereductionattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sizereductionattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereductionattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereductiontexture(mut self, val: &str) -> Self {
        self.params.insert("sizereductiontexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sizereductiontexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereductiontexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizereductiontextureprim(mut self, val: &str) -> Self {
        self.params.insert("sizereductiontextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sizereductiontextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizereductiontextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedbackattrib(mut self, val: &str) -> Self {
        self.params.insert("goalfeedbackattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_goalfeedbackattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedbackattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedbacktexture(mut self, val: &str) -> Self {
        self.params.insert("goalfeedbacktexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_goalfeedbacktexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedbacktexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goalfeedbacktextureprim(mut self, val: &str) -> Self {
        self.params.insert("goalfeedbacktextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_goalfeedbacktextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("goalfeedbacktextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductioncurveattrib(mut self, val: &str) -> Self {
        self.params.insert("tightnessreductioncurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnessreductioncurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductioncurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductionattrib(mut self, val: &str) -> Self {
        self.params.insert("tightnessreductionattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnessreductionattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductionattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductiontexture(mut self, val: &str) -> Self {
        self.params.insert("tightnessreductiontexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnessreductiontexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductiontexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessreductiontextureprim(mut self, val: &str) -> Self {
        self.params.insert("tightnessreductiontextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnessreductiontextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessreductiontextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlampcurveattrib(mut self, val: &str) -> Self {
        self.params.insert("curlampcurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_curlampcurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlampcurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlampattrib(mut self, val: &str) -> Self {
        self.params.insert("curlampattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_curlampattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlampattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlamptexture(mut self, val: &str) -> Self {
        self.params.insert("curlamptexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_curlamptexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlamptexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlamptextureprim(mut self, val: &str) -> Self {
        self.params.insert("curlamptextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_curlamptextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlamptextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqcurveattrib(mut self, val: &str) -> Self {
        self.params.insert("curlfreqcurveattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_curlfreqcurveattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqcurveattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqattrib(mut self, val: &str) -> Self {
        self.params.insert("curlfreqattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_curlfreqattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqtexture(mut self, val: &str) -> Self {
        self.params.insert("curlfreqtexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_curlfreqtexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqtexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curlfreqtextureprim(mut self, val: &str) -> Self {
        self.params.insert("curlfreqtextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_curlfreqtextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("curlfreqtextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpidinputattrib(mut self, val: &str) -> Self {
        self.params.insert("clumpidinputattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpidinputattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpidinputattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpidoutputattrib(mut self, val: &str) -> Self {
        self.params.insert("clumpidoutputattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpidoutputattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpidoutputattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_orientinputattrib(mut self, val: &str) -> Self {
        self.params.insert("orientinputattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_orientinputattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("orientinputattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tightnessoutputattrib(mut self, val: &str) -> Self {
        self.params.insert("tightnessoutputattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tightnessoutputattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("tightnessoutputattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumppointattribs(mut self, val: &str) -> Self {
        self.params.insert("clumppointattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumppointattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumppointattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpprimattribs(mut self, val: &str) -> Self {
        self.params.insert("clumpprimattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpprimattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpprimattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_legacymasking(mut self, val: bool) -> Self {
        self.params.insert("legacymasking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_legacymasking_expr(mut self, expr: &str) -> Self {
        self.params.insert("legacymasking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useskinmask(mut self, val: bool) -> Self {
        self.params.insert("useskinmask".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useskinmask_expr(mut self, expr: &str) -> Self {
        self.params.insert("useskinmask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_searchbeyondradius(mut self, val: bool) -> Self {
        self.params.insert("searchbeyondradius".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_searchbeyondradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("searchbeyondradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpwithinclumps(mut self, val: bool) -> Self {
        self.params.insert("clumpwithinclumps".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_clumpwithinclumps_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpwithinclumps".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_preservelength(mut self, val: bool) -> Self {
        self.params.insert("preservelength".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_preservelength_expr(mut self, expr: &str) -> Self {
        self.params.insert("preservelength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_accuratebundling(mut self, val: bool) -> Self {
        self.params.insert("accuratebundling".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_accuratebundling_expr(mut self, expr: &str) -> Self {
        self.params.insert("accuratebundling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_enablecurling(mut self, val: bool) -> Self {
        self.params.insert("enablecurling".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_enablecurling_expr(mut self, expr: &str) -> Self {
        self.params.insert("enablecurling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_createclumpidattrib(mut self, val: bool) -> Self {
        self.params.insert("createclumpidattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_createclumpidattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("createclumpidattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useorientinputattrib(mut self, val: bool) -> Self {
        self.params.insert("useorientinputattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useorientinputattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("useorientinputattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_createtightnessattrib(mut self, val: bool) -> Self {
        self.params.insert("createtightnessattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_createtightnessattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("createtightnessattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHairclumpcore {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "hairclumpcore"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenMode {
    ScatterOnSurface = 0,
    PerPoint = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenDensityoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenSkinguidemode {
    MatchByGuideId = 0,
    /// Weight Array Pair (guides & weights)
    WeightArrayPairGuidesWeights = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenGuideblendmethod {
    LinearBlend = 0,
    ExtrudeAndBlend = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenInfluenceradiusoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenInfluenceradiusoptions {
    Fit = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenInfluencedecayoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenInfluencedecayoptions {
    Fit = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenMaxguidecountoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenMaxguidecountoptions {
    Fit = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenMaxguideangleoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenMaxguideangleoptions {
    Fit = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenClumpcrossoveroverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenClumpcrossoveroptions {
    Fit = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenUnguidedlengthoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenUnguidedlengthoptions {
    Fit = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgenThicknessoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
}

#[derive(Debug, Clone)]
pub struct SopHairgen {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHairgen {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Rest Skin or Points"
    pub fn set_input_rest_skin_or_points<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Rest Skin or Points" and specifies the output index of the target node.
    pub fn set_input_rest_skin_or_points_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Guides"
    pub fn set_input_guides<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Guides" and specifies the output index of the target node.
    pub fn set_input_guides_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Animated Skin or Points"
    pub fn set_input_animated_skin_or_points<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Animated Skin or Points" and specifies the output index of the target node.
    pub fn set_input_animated_skin_or_points_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Volumes"
    pub fn set_input_volumes<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Volumes" and specifies the output index of the target node.
    pub fn set_input_volumes_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: "Guide Interpolation Mesh"
    pub fn set_input_guide_interpolation_mesh<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "Guide Interpolation Mesh" and specifies the output index of the target node.
    pub fn set_input_guide_interpolation_mesh_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert("density".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert("density".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scatterseed(mut self, val: f32) -> Self {
        self.params.insert("scatterseed".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scatterseed_expr(mut self, expr: &str) -> Self {
        self.params.insert("scatterseed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pruningratio(mut self, val: f32) -> Self {
        self.params.insert("pruningratio".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pruningratio_expr(mut self, expr: &str) -> Self {
        self.params.insert("pruningratio".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influenceradius(mut self, val: f32) -> Self {
        self.params.insert("influenceradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_influenceradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("influenceradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influencedecay(mut self, val: f32) -> Self {
        self.params.insert("influencedecay".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_influencedecay_expr(mut self, expr: &str) -> Self {
        self.params.insert("influencedecay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguideangle(mut self, val: f32) -> Self {
        self.params.insert("maxguideangle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maxguideangle_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguideangle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpcrossover(mut self, val: f32) -> Self {
        self.params.insert("clumpcrossover".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_clumpcrossover_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpcrossover".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedlength(mut self, val: f32) -> Self {
        self.params.insert("unguidedlength".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_unguidedlength_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedlength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedminlength(mut self, val: f32) -> Self {
        self.params.insert("unguidedminlength".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_unguidedminlength_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedminlength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_thickness(mut self, val: f32) -> Self {
        self.params.insert("thickness".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_thickness_expr(mut self, expr: &str) -> Self {
        self.params.insert("thickness".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_influenceradiusinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("influenceradiusinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_influenceradiusinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("influenceradiusinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influenceradiusoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("influenceradiusoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_influenceradiusoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("influenceradiusoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influencedecayinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("influencedecayinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_influencedecayinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("influencedecayinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influencedecayoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("influencedecayoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_influencedecayoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("influencedecayoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguidecountinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("maxguidecountinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_maxguidecountinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguidecountinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguidecountoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("maxguidecountoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_maxguidecountoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguidecountoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguideangleinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("maxguideangleinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_maxguideangleinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguideangleinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguideangleoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("maxguideangleoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_maxguideangleoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguideangleoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpcrossoverinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("clumpcrossoverinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_clumpcrossoverinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpcrossoverinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpcrossoveroutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("clumpcrossoveroutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_clumpcrossoveroutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpcrossoveroutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedlengthinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("unguidedlengthinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_unguidedlengthinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedlengthinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedlengthoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("unguidedlengthoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_unguidedlengthoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedlengthoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_bboxsize(mut self, val: [f32; 3]) -> Self {
        self.params.insert("bboxsize".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_bboxsize_expr(mut self, expr: &str) -> Self {
        self.params.insert("bboxsize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bboxcenter(mut self, val: [f32; 3]) -> Self {
        self.params.insert("bboxcenter".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_bboxcenter_expr(mut self, expr: &str) -> Self {
        self.params.insert("bboxcenter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_count(mut self, val: i32) -> Self {
        self.params.insert("count".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_count_expr(mut self, expr: &str) -> Self {
        self.params.insert("count".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scatterrelaxiterations(mut self, val: i32) -> Self {
        self.params.insert("scatterrelaxiterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_scatterrelaxiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("scatterrelaxiterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguidecount(mut self, val: i32) -> Self {
        self.params.insert("maxguidecount".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_maxguidecount_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguidecount".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedsegments(mut self, val: i32) -> Self {
        self.params.insert("unguidedsegments".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_unguidedsegments_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedsegments".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_mode(mut self, val: SopHairgenMode) -> Self {
        self.params.insert("mode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert("mode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_densityoverride(mut self, val: SopHairgenDensityoverride) -> Self {
        self.params.insert("densityoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_densityoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("densityoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_skinguidemode(mut self, val: SopHairgenSkinguidemode) -> Self {
        self.params.insert("skinguidemode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_skinguidemode_expr(mut self, expr: &str) -> Self {
        self.params.insert("skinguidemode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guideblendmethod(mut self, val: SopHairgenGuideblendmethod) -> Self {
        self.params.insert("guideblendmethod".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_guideblendmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert("guideblendmethod".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influenceradiusoverride(mut self, val: SopHairgenInfluenceradiusoverride) -> Self {
        self.params.insert("influenceradiusoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_influenceradiusoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("influenceradiusoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influenceradiusoptions(mut self, val: SopHairgenInfluenceradiusoptions) -> Self {
        self.params.insert("influenceradiusoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_influenceradiusoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("influenceradiusoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influencedecayoverride(mut self, val: SopHairgenInfluencedecayoverride) -> Self {
        self.params.insert("influencedecayoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_influencedecayoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("influencedecayoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influencedecayoptions(mut self, val: SopHairgenInfluencedecayoptions) -> Self {
        self.params.insert("influencedecayoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_influencedecayoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("influencedecayoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguidecountoverride(mut self, val: SopHairgenMaxguidecountoverride) -> Self {
        self.params.insert("maxguidecountoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_maxguidecountoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguidecountoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguidecountoptions(mut self, val: SopHairgenMaxguidecountoptions) -> Self {
        self.params.insert("maxguidecountoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_maxguidecountoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguidecountoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguideangleoverride(mut self, val: SopHairgenMaxguideangleoverride) -> Self {
        self.params.insert("maxguideangleoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_maxguideangleoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguideangleoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguideangleoptions(mut self, val: SopHairgenMaxguideangleoptions) -> Self {
        self.params.insert("maxguideangleoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_maxguideangleoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguideangleoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpcrossoveroverride(mut self, val: SopHairgenClumpcrossoveroverride) -> Self {
        self.params.insert("clumpcrossoveroverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_clumpcrossoveroverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpcrossoveroverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpcrossoveroptions(mut self, val: SopHairgenClumpcrossoveroptions) -> Self {
        self.params.insert("clumpcrossoveroptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_clumpcrossoveroptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpcrossoveroptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedlengthoverride(mut self, val: SopHairgenUnguidedlengthoverride) -> Self {
        self.params.insert("unguidedlengthoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_unguidedlengthoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedlengthoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedlengthoptions(mut self, val: SopHairgenUnguidedlengthoptions) -> Self {
        self.params.insert("unguidedlengthoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_unguidedlengthoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedlengthoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_thicknessoverride(mut self, val: SopHairgenThicknessoverride) -> Self {
        self.params.insert("thicknessoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_thicknessoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("thicknessoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_hairprofile(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("hairprofile".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_hairprofile_expr(mut self, expr: &str) -> Self {
        self.params.insert("hairprofile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_densityattrib(mut self, val: &str) -> Self {
        self.params.insert("densityattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_densityattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("densityattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_densitytexture(mut self, val: &str) -> Self {
        self.params.insert("densitytexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_densitytexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("densitytexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guidegroup(mut self, val: &str) -> Self {
        self.params.insert("guidegroup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_guidegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("guidegroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influenceradiusattrib(mut self, val: &str) -> Self {
        self.params.insert("influenceradiusattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_influenceradiusattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("influenceradiusattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influenceradiustexture(mut self, val: &str) -> Self {
        self.params.insert("influenceradiustexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_influenceradiustexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("influenceradiustexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influenceradiustextureprim(mut self, val: &str) -> Self {
        self.params.insert("influenceradiustextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_influenceradiustextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("influenceradiustextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influencedecayattrib(mut self, val: &str) -> Self {
        self.params.insert("influencedecayattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_influencedecayattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("influencedecayattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influencedecaytexture(mut self, val: &str) -> Self {
        self.params.insert("influencedecaytexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_influencedecaytexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("influencedecaytexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influencedecaytextureprim(mut self, val: &str) -> Self {
        self.params.insert("influencedecaytextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_influencedecaytextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("influencedecaytextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguidecountattrib(mut self, val: &str) -> Self {
        self.params.insert("maxguidecountattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_maxguidecountattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguidecountattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguidecounttexture(mut self, val: &str) -> Self {
        self.params.insert("maxguidecounttexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_maxguidecounttexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguidecounttexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguidecounttextureprim(mut self, val: &str) -> Self {
        self.params.insert("maxguidecounttextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_maxguidecounttextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguidecounttextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguideangleattrib(mut self, val: &str) -> Self {
        self.params.insert("maxguideangleattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_maxguideangleattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguideangleattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguideangletexture(mut self, val: &str) -> Self {
        self.params.insert("maxguideangletexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_maxguideangletexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguideangletexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguideangletextureprim(mut self, val: &str) -> Self {
        self.params.insert("maxguideangletextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_maxguideangletextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguideangletextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpcrossoverattrib(mut self, val: &str) -> Self {
        self.params.insert("clumpcrossoverattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpcrossoverattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpcrossoverattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpcrossovertexture(mut self, val: &str) -> Self {
        self.params.insert("clumpcrossovertexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpcrossovertexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpcrossovertexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpcrossovertextureprim(mut self, val: &str) -> Self {
        self.params.insert("clumpcrossovertextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpcrossovertextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpcrossovertextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpidattrib(mut self, val: &str) -> Self {
        self.params.insert("clumpidattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpidattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpidattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_initdirattrib(mut self, val: &str) -> Self {
        self.params.insert("initdirattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_initdirattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("initdirattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedlengthattrib(mut self, val: &str) -> Self {
        self.params.insert("unguidedlengthattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_unguidedlengthattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedlengthattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedlengthtexture(mut self, val: &str) -> Self {
        self.params.insert("unguidedlengthtexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_unguidedlengthtexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedlengthtexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedlengthtextureprim(mut self, val: &str) -> Self {
        self.params.insert("unguidedlengthtextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_unguidedlengthtextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedlengthtextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_thicknessattrib(mut self, val: &str) -> Self {
        self.params.insert("thicknessattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_thicknessattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("thicknessattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_thicknessskinattrib(mut self, val: &str) -> Self {
        self.params.insert("thicknessskinattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_thicknessskinattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("thicknessskinattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_thicknesstexture(mut self, val: &str) -> Self {
        self.params.insert("thicknesstexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_thicknesstexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("thicknesstexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pointattribs(mut self, val: &str) -> Self {
        self.params.insert("pointattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pointattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("pointattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vertattribs(mut self, val: &str) -> Self {
        self.params.insert("vertattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vertattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("vertattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_primattribs(mut self, val: &str) -> Self {
        self.params.insert("primattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_primattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("primattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_detailattribs(mut self, val: &str) -> Self {
        self.params.insert("detailattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_detailattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("detailattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guidepointattribs(mut self, val: &str) -> Self {
        self.params.insert("guidepointattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_guidepointattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("guidepointattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guideprimattribs(mut self, val: &str) -> Self {
        self.params.insert("guideprimattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_guideprimattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("guideprimattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guideindexattrib_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("guideindexattrib{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_guideindexattrib_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("guideindexattrib{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guideweightattrib_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("guideweightattrib{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_guideweightattrib_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("guideweightattrib{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_forcecount(mut self, val: bool) -> Self {
        self.params.insert("forcecount".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_forcecount_expr(mut self, expr: &str) -> Self {
        self.params.insert("forcecount".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_randomizeorder(mut self, val: bool) -> Self {
        self.params.insert("randomizeorder".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_randomizeorder_expr(mut self, expr: &str) -> Self {
        self.params.insert("randomizeorder".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_relaxusenormals(mut self, val: bool) -> Self {
        self.params.insert("relaxusenormals".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_relaxusenormals_expr(mut self, expr: &str) -> Self {
        self.params.insert("relaxusenormals".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_limittobbox(mut self, val: bool) -> Self {
        self.params.insert("limittobbox".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_limittobbox_expr(mut self, expr: &str) -> Self {
        self.params.insert("limittobbox".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_prune(mut self, val: bool) -> Self {
        self.params.insert("prune".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_prune_expr(mut self, expr: &str) -> Self {
        self.params.insert("prune".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_prunethicken(mut self, val: bool) -> Self {
        self.params.insert("prunethicken".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_prunethicken_expr(mut self, expr: &str) -> Self {
        self.params.insert("prunethicken".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_prunestable(mut self, val: bool) -> Self {
        self.params.insert("prunestable".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_prunestable_expr(mut self, expr: &str) -> Self {
        self.params.insert("prunestable".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useguides(mut self, val: bool) -> Self {
        self.params.insert("useguides".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useguides_expr(mut self, expr: &str) -> Self {
        self.params.insert("useguides".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uniformguidesegments(mut self, val: bool) -> Self {
        self.params.insert("uniformguidesegments".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_uniformguidesegments_expr(mut self, expr: &str) -> Self {
        self.params.insert("uniformguidesegments".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_skininterp(mut self, val: bool) -> Self {
        self.params.insert("skininterp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_skininterp_expr(mut self, expr: &str) -> Self {
        self.params.insert("skininterp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useinterpmesh(mut self, val: bool) -> Self {
        self.params.insert("useinterpmesh".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useinterpmesh_expr(mut self, expr: &str) -> Self {
        self.params.insert("useinterpmesh".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_createweightattribs(mut self, val: bool) -> Self {
        self.params.insert("createweightattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_createweightattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("createweightattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_growunguided(mut self, val: bool) -> Self {
        self.params.insert("growunguided".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_growunguided_expr(mut self, expr: &str) -> Self {
        self.params.insert("growunguided".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useinitdirattrib(mut self, val: bool) -> Self {
        self.params.insert("useinitdirattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useinitdirattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("useinitdirattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputthicknessattrib(mut self, val: bool) -> Self {
        self.params.insert("outputthicknessattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_outputthicknessattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputthicknessattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHairgen {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "hairgen"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgencoreMode {
    ScatterOnSurface = 0,
    PerPoint = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgencoreSkinguidemode {
    MatchByGuideId = 0,
    /// Weight Array Pair (guides & weights)
    WeightArrayPairGuidesWeights = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgencoreGuideblendmethod {
    LinearBlend = 0,
    ExtrudeAndBlend = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgencoreInfluenceradiusoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgencoreInfluenceradiusoptions {
    Fit = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgencoreInfluencedecayoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgencoreInfluencedecayoptions {
    Fit = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgencoreMaxguidecountoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgencoreMaxguidecountoptions {
    Fit = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgencoreMaxguideangleoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgencoreMaxguideangleoptions {
    Fit = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgencoreClumpcrossoveroverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgencoreClumpcrossoveroptions {
    Fit = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgencoreUnguidedlengthoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
    TexturePrimitive = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgencoreUnguidedlengthoptions {
    Fit = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHairgencoreThicknessoverride {
    NoOverride = 0,
    SkinAttribute = 1,
    Texture = 2,
}

#[derive(Debug, Clone)]
pub struct SopHairgencore {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHairgencore {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Skin"
    pub fn set_input_skin<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Guides"
    pub fn set_input_guides<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Guides" and specifies the output index of the target node.
    pub fn set_input_guides_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Root Positions"
    pub fn set_input_root_positions<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Root Positions" and specifies the output index of the target node.
    pub fn set_input_root_positions_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Volumes"
    pub fn set_input_volumes<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Volumes" and specifies the output index of the target node.
    pub fn set_input_volumes_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: "Interpolation Mesh"
    pub fn set_input_interpolation_mesh<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "Interpolation Mesh" and specifies the output index of the target node.
    pub fn set_input_interpolation_mesh_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_pruningratio(mut self, val: f32) -> Self {
        self.params.insert("pruningratio".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pruningratio_expr(mut self, expr: &str) -> Self {
        self.params.insert("pruningratio".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influenceradius(mut self, val: f32) -> Self {
        self.params.insert("influenceradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_influenceradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("influenceradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influencedecay(mut self, val: f32) -> Self {
        self.params.insert("influencedecay".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_influencedecay_expr(mut self, expr: &str) -> Self {
        self.params.insert("influencedecay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguideangle(mut self, val: f32) -> Self {
        self.params.insert("maxguideangle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maxguideangle_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguideangle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpcrossover(mut self, val: f32) -> Self {
        self.params.insert("clumpcrossover".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_clumpcrossover_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpcrossover".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedlength(mut self, val: f32) -> Self {
        self.params.insert("unguidedlength".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_unguidedlength_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedlength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedminlength(mut self, val: f32) -> Self {
        self.params.insert("unguidedminlength".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_unguidedminlength_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedminlength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_thickness(mut self, val: f32) -> Self {
        self.params.insert("thickness".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_thickness_expr(mut self, expr: &str) -> Self {
        self.params.insert("thickness".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_influenceradiusinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("influenceradiusinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_influenceradiusinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("influenceradiusinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influenceradiusoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("influenceradiusoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_influenceradiusoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("influenceradiusoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influencedecayinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("influencedecayinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_influencedecayinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("influencedecayinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influencedecayoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("influencedecayoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_influencedecayoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("influencedecayoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguidecountinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("maxguidecountinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_maxguidecountinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguidecountinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguidecountoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("maxguidecountoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_maxguidecountoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguidecountoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguideangleinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("maxguideangleinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_maxguideangleinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguideangleinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguideangleoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("maxguideangleoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_maxguideangleoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguideangleoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpcrossoverinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("clumpcrossoverinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_clumpcrossoverinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpcrossoverinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpcrossoveroutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("clumpcrossoveroutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_clumpcrossoveroutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpcrossoveroutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedlengthinrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("unguidedlengthinrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_unguidedlengthinrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedlengthinrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedlengthoutrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("unguidedlengthoutrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_unguidedlengthoutrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedlengthoutrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_bboxsize(mut self, val: [f32; 3]) -> Self {
        self.params.insert("bboxsize".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_bboxsize_expr(mut self, expr: &str) -> Self {
        self.params.insert("bboxsize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bboxcenter(mut self, val: [f32; 3]) -> Self {
        self.params.insert("bboxcenter".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_bboxcenter_expr(mut self, expr: &str) -> Self {
        self.params.insert("bboxcenter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_maxguidecount(mut self, val: i32) -> Self {
        self.params.insert("maxguidecount".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_maxguidecount_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguidecount".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedsegments(mut self, val: i32) -> Self {
        self.params.insert("unguidedsegments".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_unguidedsegments_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedsegments".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_mode(mut self, val: SopHairgencoreMode) -> Self {
        self.params.insert("mode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert("mode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_skinguidemode(mut self, val: SopHairgencoreSkinguidemode) -> Self {
        self.params.insert("skinguidemode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_skinguidemode_expr(mut self, expr: &str) -> Self {
        self.params.insert("skinguidemode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guideblendmethod(mut self, val: SopHairgencoreGuideblendmethod) -> Self {
        self.params.insert("guideblendmethod".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_guideblendmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert("guideblendmethod".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influenceradiusoverride(mut self, val: SopHairgencoreInfluenceradiusoverride) -> Self {
        self.params.insert("influenceradiusoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_influenceradiusoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("influenceradiusoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influenceradiusoptions(mut self, val: SopHairgencoreInfluenceradiusoptions) -> Self {
        self.params.insert("influenceradiusoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_influenceradiusoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("influenceradiusoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influencedecayoverride(mut self, val: SopHairgencoreInfluencedecayoverride) -> Self {
        self.params.insert("influencedecayoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_influencedecayoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("influencedecayoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influencedecayoptions(mut self, val: SopHairgencoreInfluencedecayoptions) -> Self {
        self.params.insert("influencedecayoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_influencedecayoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("influencedecayoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguidecountoverride(mut self, val: SopHairgencoreMaxguidecountoverride) -> Self {
        self.params.insert("maxguidecountoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_maxguidecountoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguidecountoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguidecountoptions(mut self, val: SopHairgencoreMaxguidecountoptions) -> Self {
        self.params.insert("maxguidecountoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_maxguidecountoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguidecountoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguideangleoverride(mut self, val: SopHairgencoreMaxguideangleoverride) -> Self {
        self.params.insert("maxguideangleoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_maxguideangleoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguideangleoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguideangleoptions(mut self, val: SopHairgencoreMaxguideangleoptions) -> Self {
        self.params.insert("maxguideangleoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_maxguideangleoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguideangleoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpcrossoveroverride(mut self, val: SopHairgencoreClumpcrossoveroverride) -> Self {
        self.params.insert("clumpcrossoveroverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_clumpcrossoveroverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpcrossoveroverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpcrossoveroptions(mut self, val: SopHairgencoreClumpcrossoveroptions) -> Self {
        self.params.insert("clumpcrossoveroptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_clumpcrossoveroptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpcrossoveroptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedlengthoverride(mut self, val: SopHairgencoreUnguidedlengthoverride) -> Self {
        self.params.insert("unguidedlengthoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_unguidedlengthoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedlengthoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedlengthoptions(mut self, val: SopHairgencoreUnguidedlengthoptions) -> Self {
        self.params.insert("unguidedlengthoptions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_unguidedlengthoptions_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedlengthoptions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_thicknessoverride(mut self, val: SopHairgencoreThicknessoverride) -> Self {
        self.params.insert("thicknessoverride".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_thicknessoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("thicknessoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_hairprofile(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("hairprofile".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_hairprofile_expr(mut self, expr: &str) -> Self {
        self.params.insert("hairprofile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uvattrib(mut self, val: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_uvattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("uvattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_skininterpguidesattrib(mut self, val: &str) -> Self {
        self.params.insert("skininterpguidesattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_skininterpguidesattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("skininterpguidesattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_skininterpweightsattrib(mut self, val: &str) -> Self {
        self.params.insert("skininterpweightsattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_skininterpweightsattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("skininterpweightsattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guidegroup(mut self, val: &str) -> Self {
        self.params.insert("guidegroup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_guidegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert("guidegroup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influenceradiusattrib(mut self, val: &str) -> Self {
        self.params.insert("influenceradiusattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_influenceradiusattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("influenceradiusattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influenceradiustexture(mut self, val: &str) -> Self {
        self.params.insert("influenceradiustexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_influenceradiustexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("influenceradiustexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influenceradiustextureprim(mut self, val: &str) -> Self {
        self.params.insert("influenceradiustextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_influenceradiustextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("influenceradiustextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influencedecayattrib(mut self, val: &str) -> Self {
        self.params.insert("influencedecayattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_influencedecayattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("influencedecayattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influencedecaytexture(mut self, val: &str) -> Self {
        self.params.insert("influencedecaytexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_influencedecaytexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("influencedecaytexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_influencedecaytextureprim(mut self, val: &str) -> Self {
        self.params.insert("influencedecaytextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_influencedecaytextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("influencedecaytextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguidecountattrib(mut self, val: &str) -> Self {
        self.params.insert("maxguidecountattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_maxguidecountattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguidecountattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguidecounttexture(mut self, val: &str) -> Self {
        self.params.insert("maxguidecounttexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_maxguidecounttexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguidecounttexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguidecounttextureprim(mut self, val: &str) -> Self {
        self.params.insert("maxguidecounttextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_maxguidecounttextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguidecounttextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguideangleattrib(mut self, val: &str) -> Self {
        self.params.insert("maxguideangleattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_maxguideangleattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguideangleattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguideangletexture(mut self, val: &str) -> Self {
        self.params.insert("maxguideangletexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_maxguideangletexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguideangletexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxguideangletextureprim(mut self, val: &str) -> Self {
        self.params.insert("maxguideangletextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_maxguideangletextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxguideangletextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpcrossoverattrib(mut self, val: &str) -> Self {
        self.params.insert("clumpcrossoverattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpcrossoverattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpcrossoverattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpcrossovertexture(mut self, val: &str) -> Self {
        self.params.insert("clumpcrossovertexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpcrossovertexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpcrossovertexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpcrossovertextureprim(mut self, val: &str) -> Self {
        self.params.insert("clumpcrossovertextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpcrossovertextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpcrossovertextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_initdirattrib(mut self, val: &str) -> Self {
        self.params.insert("initdirattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_initdirattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("initdirattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedlengthattrib(mut self, val: &str) -> Self {
        self.params.insert("unguidedlengthattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_unguidedlengthattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedlengthattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedlengthtexture(mut self, val: &str) -> Self {
        self.params.insert("unguidedlengthtexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_unguidedlengthtexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedlengthtexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unguidedlengthtextureprim(mut self, val: &str) -> Self {
        self.params.insert("unguidedlengthtextureprim".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_unguidedlengthtextureprim_expr(mut self, expr: &str) -> Self {
        self.params.insert("unguidedlengthtextureprim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clumpidattrib(mut self, val: &str) -> Self {
        self.params.insert("clumpidattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clumpidattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("clumpidattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_thicknessattrib(mut self, val: &str) -> Self {
        self.params.insert("thicknessattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_thicknessattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("thicknessattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_thicknessskinattrib(mut self, val: &str) -> Self {
        self.params.insert("thicknessskinattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_thicknessskinattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("thicknessskinattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_thicknesstexture(mut self, val: &str) -> Self {
        self.params.insert("thicknesstexture".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_thicknesstexture_expr(mut self, expr: &str) -> Self {
        self.params.insert("thicknesstexture".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pointattribs(mut self, val: &str) -> Self {
        self.params.insert("pointattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pointattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("pointattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vertattribs(mut self, val: &str) -> Self {
        self.params.insert("vertattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vertattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("vertattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_primattribs(mut self, val: &str) -> Self {
        self.params.insert("primattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_primattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("primattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_detailattribs(mut self, val: &str) -> Self {
        self.params.insert("detailattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_detailattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("detailattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guidepointattribs(mut self, val: &str) -> Self {
        self.params.insert("guidepointattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_guidepointattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("guidepointattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guideprimattribs(mut self, val: &str) -> Self {
        self.params.insert("guideprimattribs".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_guideprimattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("guideprimattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guideindexattrib_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("guideindexattrib{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_guideindexattrib_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("guideindexattrib{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guideweightattrib_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("guideweightattrib{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_guideweightattrib_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("guideweightattrib{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_limittobbox(mut self, val: bool) -> Self {
        self.params.insert("limittobbox".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_limittobbox_expr(mut self, expr: &str) -> Self {
        self.params.insert("limittobbox".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_prune(mut self, val: bool) -> Self {
        self.params.insert("prune".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_prune_expr(mut self, expr: &str) -> Self {
        self.params.insert("prune".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_prunethicken(mut self, val: bool) -> Self {
        self.params.insert("prunethicken".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_prunethicken_expr(mut self, expr: &str) -> Self {
        self.params.insert("prunethicken".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useguides(mut self, val: bool) -> Self {
        self.params.insert("useguides".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useguides_expr(mut self, expr: &str) -> Self {
        self.params.insert("useguides".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uniformguidesegments(mut self, val: bool) -> Self {
        self.params.insert("uniformguidesegments".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_uniformguidesegments_expr(mut self, expr: &str) -> Self {
        self.params.insert("uniformguidesegments".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_skininterp(mut self, val: bool) -> Self {
        self.params.insert("skininterp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_skininterp_expr(mut self, expr: &str) -> Self {
        self.params.insert("skininterp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useinterpmesh(mut self, val: bool) -> Self {
        self.params.insert("useinterpmesh".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useinterpmesh_expr(mut self, expr: &str) -> Self {
        self.params.insert("useinterpmesh".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_growunguided(mut self, val: bool) -> Self {
        self.params.insert("growunguided".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_growunguided_expr(mut self, expr: &str) -> Self {
        self.params.insert("growunguided".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useinitdirattrib(mut self, val: bool) -> Self {
        self.params.insert("useinitdirattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useinitdirattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("useinitdirattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_createweightattribs(mut self, val: bool) -> Self {
        self.params.insert("createweightattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_createweightattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("createweightattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_createrestrootattrib(mut self, val: bool) -> Self {
        self.params.insert("createrestrootattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_createrestrootattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("createrestrootattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputthicknessattrib(mut self, val: bool) -> Self {
        self.params.insert("outputthicknessattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_outputthicknessattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputthicknessattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHairgencore {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "hairgencore"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone)]
pub struct SopHairgrowthfield {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHairgrowthfield {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Guides"
    pub fn set_input_guides<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Guides" and specifies the output index of the target node.
    pub fn set_input_guides_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Skin VDB"
    pub fn set_input_skin_vdb<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Skin VDB" and specifies the output index of the target node.
    pub fn set_input_skin_vdb_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Hair Mask"
    pub fn set_input_hair_mask<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Hair Mask" and specifies the output index of the target node.
    pub fn set_input_hair_mask_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_growthfieldquality(mut self, val: f32) -> Self {
        self.params.insert("growthfieldquality".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_growthfieldquality_expr(mut self, expr: &str) -> Self {
        self.params.insert("growthfieldquality".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scalpvoxelsize(mut self, val: f32) -> Self {
        self.params.insert("scalpvoxelsize".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scalpvoxelsize_expr(mut self, expr: &str) -> Self {
        self.params.insert("scalpvoxelsize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guidespacing(mut self, val: f32) -> Self {
        self.params.insert("guidespacing".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_guidespacing_expr(mut self, expr: &str) -> Self {
        self.params.insert("guidespacing".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guidedensityscale(mut self, val: f32) -> Self {
        self.params.insert("guidedensityscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_guidedensityscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("guidedensityscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guiderootpercent(mut self, val: f32) -> Self {
        self.params.insert("guiderootpercent".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_guiderootpercent_expr(mut self, expr: &str) -> Self {
        self.params.insert("guiderootpercent".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maskinfluence(mut self, val: f32) -> Self {
        self.params.insert("maskinfluence".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maskinfluence_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskinfluence".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strokeinfluence(mut self, val: f32) -> Self {
        self.params.insert("strokeinfluence".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_strokeinfluence_expr(mut self, expr: &str) -> Self {
        self.params.insert("strokeinfluence".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guideradius(mut self, val: f32) -> Self {
        self.params.insert("guideradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_guideradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("guideradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scalpnormalforce(mut self, val: f32) -> Self {
        self.params.insert("scalpnormalforce".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scalpnormalforce_expr(mut self, expr: &str) -> Self {
        self.params.insert("scalpnormalforce".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scalpblendwidth(mut self, val: f32) -> Self {
        self.params.insert("scalpblendwidth".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scalpblendwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert("scalpblendwidth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scalpblendwidthinner(mut self, val: f32) -> Self {
        self.params.insert("scalpblendwidthinner".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scalpblendwidthinner_expr(mut self, expr: &str) -> Self {
        self.params.insert("scalpblendwidthinner".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guideshapeeffect(mut self, val: f32) -> Self {
        self.params.insert("guideshapeeffect".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_guideshapeeffect_expr(mut self, expr: &str) -> Self {
        self.params.insert("guideshapeeffect".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guideadvectionlength(mut self, val: f32) -> Self {
        self.params.insert("guideadvectionlength".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_guideadvectionlength_expr(mut self, expr: &str) -> Self {
        self.params.insert("guideadvectionlength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_culllength(mut self, val: f32) -> Self {
        self.params.insert("culllength".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_culllength_expr(mut self, expr: &str) -> Self {
        self.params.insert("culllength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_npts(mut self, val: i32) -> Self {
        self.params.insert("npts".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_npts_expr(mut self, expr: &str) -> Self {
        self.params.insert("npts".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_emergencylimit(mut self, val: i32) -> Self {
        self.params.insert("emergencylimit".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_emergencylimit_expr(mut self, expr: &str) -> Self {
        self.params.insert("emergencylimit".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_relaxiterations(mut self, val: i32) -> Self {
        self.params.insert("relaxiterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_relaxiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("relaxiterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guideadvectionsamples(mut self, val: i32) -> Self {
        self.params.insert("guideadvectionsamples".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_guideadvectionsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert("guideadvectionsamples".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputsegs(mut self, val: i32) -> Self {
        self.params.insert("outputsegs".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_outputsegs_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputsegs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_guideshaperamp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("guideshaperamp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_guideshaperamp_expr(mut self, expr: &str) -> Self {
        self.params.insert("guideshaperamp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_haircolorramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("haircolorramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_haircolorramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("haircolorramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_scatter2_group(mut self, val: &str) -> Self {
        self.params.insert("scatter2_group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_scatter2_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("scatter2_group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_densityattrib(mut self, val: &str) -> Self {
        self.params.insert("densityattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_densityattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("densityattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_guideadvectiontype(mut self, val: &str) -> Self {
        self.params.insert("guideadvectiontype".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_guideadvectiontype_expr(mut self, expr: &str) -> Self {
        self.params.insert("guideadvectiontype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_fastedit(mut self, val: bool) -> Self {
        self.params.insert("fastedit".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_fastedit_expr(mut self, expr: &str) -> Self {
        self.params.insert("fastedit".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_displayroots(mut self, val: bool) -> Self {
        self.params.insert("displayroots".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_displayroots_expr(mut self, expr: &str) -> Self {
        self.params.insert("displayroots".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_forcetotal(mut self, val: bool) -> Self {
        self.params.insert("forcetotal".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_forcetotal_expr(mut self, expr: &str) -> Self {
        self.params.insert("forcetotal".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usedensityattrib(mut self, val: bool) -> Self {
        self.params.insert("usedensityattrib".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usedensityattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("usedensityattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useemergencylimit(mut self, val: bool) -> Self {
        self.params.insert("useemergencylimit".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useemergencylimit_expr(mut self, expr: &str) -> Self {
        self.params.insert("useemergencylimit".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_exportvectorfield(mut self, val: bool) -> Self {
        self.params.insert("exportvectorfield".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_exportvectorfield_expr(mut self, expr: &str) -> Self {
        self.params.insert("exportvectorfield".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_addroot(mut self, val: bool) -> Self {
        self.params.insert("addroot".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_addroot_expr(mut self, expr: &str) -> Self {
        self.params.insert("addroot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cullguideswitch(mut self, val: bool) -> Self {
        self.params.insert("cullguideswitch".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_cullguideswitch_expr(mut self, expr: &str) -> Self {
        self.params.insert("cullguideswitch".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_addcolor(mut self, val: bool) -> Self {
        self.params.insert("addcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_addcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("addcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHairgrowthfield {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "hairgrowthfield"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone)]
pub struct SopHeatgeodesic {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeatgeodesic {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Geometry where distances are measured"
    pub fn set_input_geometry_where_distances_are_measured<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry where distances are measured" and specifies the output index of the target node.
    pub fn set_input_geometry_where_distances_are_measured_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_smoothing(mut self, val: f32) -> Self {
        self.params.insert("smoothing".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_smoothing_expr(mut self, expr: &str) -> Self {
        self.params.insert("smoothing".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_boundarybalance(mut self, val: f32) -> Self {
        self.params.insert("boundarybalance".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_boundarybalance_expr(mut self, expr: &str) -> Self {
        self.params.insert("boundarybalance".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_srcpoints(mut self, val: &str) -> Self {
        self.params.insert("srcpoints".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_srcpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert("srcpoints".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_attrib(mut self, val: &str) -> Self {
        self.params.insert("attrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_attrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("attrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeatgeodesic {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heatgeodesic"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldOrient {
    Xy = 0,
    Yz = 1,
    Zx = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldSampling {
    Center = 0,
    Corner = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldDivisionmode {
    ByAxis = 0,
    BySize = 1,
}

#[derive(Debug, Clone)]
pub struct SopHeightfield {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfield {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }



    // --- Float parameters ---
    pub fn with_initialheight(mut self, val: f32) -> Self {
        self.params.insert("initialheight".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_initialheight_expr(mut self, expr: &str) -> Self {
        self.params.insert("initialheight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_initialmask(mut self, val: f32) -> Self {
        self.params.insert("initialmask".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_initialmask_expr(mut self, expr: &str) -> Self {
        self.params.insert("initialmask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridspacing(mut self, val: f32) -> Self {
        self.params.insert("gridspacing".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gridspacing_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridspacing".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_size(mut self, val: [f32; 2]) -> Self {
        self.params.insert("size".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert("size".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_gridsamples(mut self, val: i32) -> Self {
        self.params.insert("gridsamples".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_gridsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridsamples".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_orient(mut self, val: SopHeightfieldOrient) -> Self {
        self.params.insert("orient".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_orient_expr(mut self, expr: &str) -> Self {
        self.params.insert("orient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sampling(mut self, val: SopHeightfieldSampling) -> Self {
        self.params.insert("sampling".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_sampling_expr(mut self, expr: &str) -> Self {
        self.params.insert("sampling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_divisionmode(mut self, val: SopHeightfieldDivisionmode) -> Self {
        self.params.insert("divisionmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_divisionmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("divisionmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfield {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldBlurMethod {
    Blur = 0,
    BoxBlur = 1,
    Expand = 2,
    Shrink = 3,
    Sharpen = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldBlurBordertype {
    UseVolume = 0,
    Constant = 1,
    Repeat = 2,
    Streak = 3,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldBlur {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldBlur {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "HeightField to Blur"
    pub fn set_input_heightfield_to_blur<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "HeightField to Blur" and specifies the output index of the target node.
    pub fn set_input_heightfield_to_blur_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask"
    pub fn set_input_mask<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask" and specifies the output index of the target node.
    pub fn set_input_mask_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_radius(mut self, val: f32) -> Self {
        self.params.insert("radius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert("radius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_borderval(mut self, val: f32) -> Self {
        self.params.insert("borderval".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_borderval_expr(mut self, expr: &str) -> Self {
        self.params.insert("borderval".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sharpenstrength(mut self, val: f32) -> Self {
        self.params.insert("sharpenstrength".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_sharpenstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert("sharpenstrength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: SopHeightfieldBlurMethod) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bordertype(mut self, val: SopHeightfieldBlurBordertype) -> Self {
        self.params.insert("bordertype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_bordertype_expr(mut self, expr: &str) -> Self {
        self.params.insert("bordertype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_layer(mut self, val: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_layer_expr(mut self, expr: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_masklayer(mut self, val: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_masklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_maskaware(mut self, val: bool) -> Self {
        self.params.insert("maskaware".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_maskaware_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskaware".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldBlur {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_blur"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldClipMaskoutput {
    ClippedLayer = 0,
    EdgeLayer = 1,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldClip {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldClip {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask"
    pub fn set_input_mask<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask" and specifies the output index of the target node.
    pub fn set_input_mask_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_computerange(mut self) -> Self {
        self.params.insert("computerange".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_minclip(mut self, val: f32) -> Self {
        self.params.insert("minclip".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_minclip_expr(mut self, expr: &str) -> Self {
        self.params.insert("minclip".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxclip(mut self, val: f32) -> Self {
        self.params.insert("maxclip".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maxclip_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxclip".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clipstrength(mut self, val: f32) -> Self {
        self.params.insert("clipstrength".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_clipstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert("clipstrength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clipscale(mut self, val: f32) -> Self {
        self.params.insert("clipscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_clipscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("clipscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_edgemaskradius(mut self, val: f32) -> Self {
        self.params.insert("edgemaskradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_edgemaskradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("edgemaskradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_maskoutput(mut self, val: SopHeightfieldClipMaskoutput) -> Self {
        self.params.insert("maskoutput".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_maskoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskoutput".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_heightlayer(mut self, val: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_heightlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_masklayer(mut self, val: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_masklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_edgelayer(mut self, val: &str) -> Self {
        self.params.insert("edgelayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_edgelayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("edgelayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cliplayer(mut self, val: &str) -> Self {
        self.params.insert("cliplayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_cliplayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("cliplayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_dominclip(mut self, val: bool) -> Self {
        self.params.insert("dominclip".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dominclip_expr(mut self, expr: &str) -> Self {
        self.params.insert("dominclip".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_domaxclip(mut self, val: bool) -> Self {
        self.params.insert("domaxclip".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_domaxclip_expr(mut self, expr: &str) -> Self {
        self.params.insert("domaxclip".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dosoftclip(mut self, val: bool) -> Self {
        self.params.insert("dosoftclip".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dosoftclip_expr(mut self, expr: &str) -> Self {
        self.params.insert("dosoftclip".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldClip {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_clip"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone)]
pub struct SopHeightfieldCopylayer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldCopylayer {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain to Copy Layers within"
    pub fn set_input_terrain_to_copy_layers_within<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain to Copy Layers within" and specifies the output index of the target node.
    pub fn set_input_terrain_to_copy_layers_within_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- String parameters ---
    pub fn with_srcname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("srcname{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_srcname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("srcname{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dstname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("dstname{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_dstname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("dstname{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_create_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("create{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_create_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("create{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_copysrc_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("copysrc{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_copysrc_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("copysrc{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_replacedst_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("replacedst{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_replacedst_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("replacedst{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldCopylayer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_copylayer"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldCropCropmode {
    Replace = 0,
    Union = 1,
    Intersect = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldCropOrient {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldCrop {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldCrop {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Bounding Objects"
    pub fn set_input_bounding_objects<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Bounding Objects" and specifies the output index of the target node.
    pub fn set_input_bounding_objects_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_cropvalue(mut self, val: f32) -> Self {
        self.params.insert("cropvalue".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_cropvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert("cropvalue".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_size(mut self, val: [f32; 2]) -> Self {
        self.params.insert("size".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert("size".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_voxelpad(mut self, val: i32) -> Self {
        self.params.insert("voxelpad".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_voxelpad_expr(mut self, expr: &str) -> Self {
        self.params.insert("voxelpad".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_cropmode(mut self, val: SopHeightfieldCropCropmode) -> Self {
        self.params.insert("cropmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_cropmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("cropmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_orient(mut self, val: SopHeightfieldCropOrient) -> Self {
        self.params.insert("orient".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_orient_expr(mut self, expr: &str) -> Self {
        self.params.insert("orient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_cropbylayer(mut self, val: &str) -> Self {
        self.params.insert("cropbylayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_cropbylayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("cropbylayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_keepbelow(mut self, val: bool) -> Self {
        self.params.insert("keepbelow".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keepbelow_expr(mut self, expr: &str) -> Self {
        self.params.insert("keepbelow".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usepoints(mut self, val: bool) -> Self {
        self.params.insert("usepoints".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usepoints_expr(mut self, expr: &str) -> Self {
        self.params.insert("usepoints".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldCrop {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_crop"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldCutoutbyobjectCombine {
    Replace = 0,
    Intersect = 1,
    Union = 2,
    Subtract = 3,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldCutoutbyobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldCutoutbyobject {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain to Cutout"
    pub fn set_input_terrain_to_cutout<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain to Cutout" and specifies the output index of the target node.
    pub fn set_input_terrain_to_cutout_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Geometry to Cutout"
    pub fn set_input_geometry_to_cutout<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Geometry to Cutout" and specifies the output index of the target node.
    pub fn set_input_geometry_to_cutout_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Menu parameters ---
    pub fn with_combine(mut self, val: SopHeightfieldCutoutbyobjectCombine) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_combine_expr(mut self, expr: &str) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_alphalayer(mut self, val: &str) -> Self {
        self.params.insert("alphalayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_alphalayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("alphalayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightlayer(mut self, val: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_heightlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_invert(mut self, val: bool) -> Self {
        self.params.insert("invert".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_invert_expr(mut self, expr: &str) -> Self {
        self.params.insert("invert".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_crop(mut self, val: bool) -> Self {
        self.params.insert("crop".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_crop_expr(mut self, expr: &str) -> Self {
        self.params.insert("crop".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldCutoutbyobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_cutoutbyobject"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone)]
pub struct SopHeightfieldDeform {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldDeform {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Geometry to Deform"
    pub fn set_input_geometry_to_deform<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Deform" and specifies the output index of the target node.
    pub fn set_input_geometry_to_deform_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Rest HeightField"
    pub fn set_input_rest_heightfield<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Rest HeightField" and specifies the output index of the target node.
    pub fn set_input_rest_heightfield_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Deformed HeightField"
    pub fn set_input_deformed_heightfield<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Deformed HeightField" and specifies the output index of the target node.
    pub fn set_input_deformed_heightfield_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_blurradius(mut self, val: f32) -> Self {
        self.params.insert("blurradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blurradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("blurradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_dorotate(mut self, val: bool) -> Self {
        self.params.insert("dorotate".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dorotate_expr(mut self, expr: &str) -> Self {
        self.params.insert("dorotate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldDeform {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_deform"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldDistortNoisetype {
    Simplex = 0,
    Curl = 1,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldDistort {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldDistort {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask"
    pub fn set_input_mask<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask" and specifies the output index of the target node.
    pub fn set_input_mask_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_amp(mut self, val: f32) -> Self {
        self.params.insert("amp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert("amp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_element_size(mut self, val: f32) -> Self {
        self.params.insert("element_size".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_element_size_expr(mut self, expr: &str) -> Self {
        self.params.insert("element_size".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rough(mut self, val: f32) -> Self {
        self.params.insert("rough".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert("rough".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_element_scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert("element_scale".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_element_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("element_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert("offset".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_maxoctave(mut self, val: i32) -> Self {
        self.params.insert("maxoctave".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_maxoctave_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxoctave".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_steps(mut self, val: i32) -> Self {
        self.params.insert("steps".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_steps_expr(mut self, expr: &str) -> Self {
        self.params.insert("steps".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_noisetype(mut self, val: SopHeightfieldDistortNoisetype) -> Self {
        self.params.insert("noisetype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_noisetype_expr(mut self, expr: &str) -> Self {
        self.params.insert("noisetype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_layer(mut self, val: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_layer_expr(mut self, expr: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_masklayer(mut self, val: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_masklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldDistort {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_distort"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldDistortbylayerDisplaceby {
    GradientOfControlLayer = 0,
    VectorControlLayer = 1,
    /// Angle, Scaled by Control Layer
    AngleScaledByControlLayer = 2,
    /// Direction, Scaled by Control Layer
    DirectionScaledByControlLayer = 3,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldDistortbylayer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldDistortbylayer {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Layers to Displace"
    pub fn set_input_layers_to_displace<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Layers to Displace" and specifies the output index of the target node.
    pub fn set_input_layers_to_displace_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Control Layer"
    pub fn set_input_control_layer<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Control Layer" and specifies the output index of the target node.
    pub fn set_input_control_layer_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_control_blur_radius(mut self, val: f32) -> Self {
        self.params.insert("control_blur_radius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_control_blur_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert("control_blur_radius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_displacescale(mut self, val: f32) -> Self {
        self.params.insert("displacescale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_displacescale_expr(mut self, expr: &str) -> Self {
        self.params.insert("displacescale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_displacement_angle(mut self, val: f32) -> Self {
        self.params.insert("displacement_angle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_displacement_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert("displacement_angle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_displaceangle(mut self, val: f32) -> Self {
        self.params.insert("displaceangle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_displaceangle_expr(mut self, expr: &str) -> Self {
        self.params.insert("displaceangle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_displacevector(mut self, val: [f32; 3]) -> Self {
        self.params.insert("displacevector".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_displacevector_expr(mut self, expr: &str) -> Self {
        self.params.insert("displacevector".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_substeps(mut self, val: i32) -> Self {
        self.params.insert("substeps".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_substeps_expr(mut self, expr: &str) -> Self {
        self.params.insert("substeps".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_displaceby(mut self, val: SopHeightfieldDistortbylayerDisplaceby) -> Self {
        self.params.insert("displaceby".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_displaceby_expr(mut self, expr: &str) -> Self {
        self.params.insert("displaceby".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_smear_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("smear_ramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_smear_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("smear_ramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_layers(mut self, val: &str) -> Self {
        self.params.insert("layers".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_layers_expr(mut self, expr: &str) -> Self {
        self.params.insert("layers".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_controllayer(mut self, val: &str) -> Self {
        self.params.insert("controllayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_controllayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("controllayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scalefield(mut self, val: &str) -> Self {
        self.params.insert("scalefield".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_scalefield_expr(mut self, expr: &str) -> Self {
        self.params.insert("scalefield".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_constant_field(mut self, val: bool) -> Self {
        self.params.insert("constant_field".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constant_field_expr(mut self, expr: &str) -> Self {
        self.params.insert("constant_field".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_should_swirl(mut self, val: bool) -> Self {
        self.params.insert("should_swirl".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_should_swirl_expr(mut self, expr: &str) -> Self {
        self.params.insert("should_swirl".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_should_smear(mut self, val: bool) -> Self {
        self.params.insert("should_smear".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_should_smear_expr(mut self, expr: &str) -> Self {
        self.params.insert("should_smear".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_smear_symmetrical(mut self, val: bool) -> Self {
        self.params.insert("smear_symmetrical".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_smear_symmetrical_expr(mut self, expr: &str) -> Self {
        self.params.insert("smear_symmetrical".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldDistortbylayer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_distortbylayer"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldDrawmaskCombine {
    Replace = 0,
    Add = 1,
    Subtract = 2,
    Difference = 3,
    Multiply = 4,
    Maximum = 5,
    Minimum = 6,
    Blend = 7,
    Edit = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldDrawmaskStrokeProjtype {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
    ScreenPlane = 3,
    Geometry = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldDrawmaskStrokeTool {
    Add = 0,
    Subtract = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldDrawmaskBlurmethod {
    Blur = 0,
    BoxBlur = 1,
    Expand = 2,
    Shrink = 3,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldDrawmask {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldDrawmask {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain to Draw Mask on"
    pub fn set_input_terrain_to_draw_mask_on<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain to Draw Mask on" and specifies the output index of the target node.
    pub fn set_input_terrain_to_draw_mask_on_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_value(mut self, val: f32) -> Self {
        self.params.insert("value".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_value_expr(mut self, expr: &str) -> Self {
        self.params.insert("value".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blurradius(mut self, val: f32) -> Self {
        self.params.insert("blurradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blurradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("blurradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_radius_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(format!("stroke{}_radius", index1), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stroke_radius_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_radius", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_opacity_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(format!("stroke{}_opacity", index1), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stroke_opacity_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_opacity", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_stroke_color_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(format!("stroke{}_color", index1), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_stroke_color_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_color", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_projcenter_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(format!("stroke{}_projcenter", index1), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_stroke_projcenter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_projcenter", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_projdir_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(format!("stroke{}_projdir", index1), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_stroke_projdir_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_projdir", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_stroke_tool_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(format!("stroke{}_tool", index1), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_stroke_tool_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_tool", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_projtype_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(format!("stroke{}_projtype", index1), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_stroke_projtype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_projtype", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_combine(mut self, val: SopHeightfieldDrawmaskCombine) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_combine_expr(mut self, expr: &str) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_projtype(mut self, val: SopHeightfieldDrawmaskStrokeProjtype) -> Self {
        self.params.insert("stroke_projtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_stroke_projtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("stroke_projtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_tool(mut self, val: SopHeightfieldDrawmaskStrokeTool) -> Self {
        self.params.insert("stroke_tool".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_stroke_tool_expr(mut self, expr: &str) -> Self {
        self.params.insert("stroke_tool".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blurmethod(mut self, val: SopHeightfieldDrawmaskBlurmethod) -> Self {
        self.params.insert("blurmethod".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_blurmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert("blurmethod".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_stroke_data_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("stroke{}_data", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_stroke_data_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_data", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_masklayer(mut self, val: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_masklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_invertmask(mut self, val: bool) -> Self {
        self.params.insert("invertmask".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_invertmask_expr(mut self, expr: &str) -> Self {
        self.params.insert("invertmask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("stroke{}_enable", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_stroke_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_enable", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldDrawmask {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_drawmask"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeErodabilitymaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeFlowmaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeBankanglemaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeCoveragemaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeSlopeinfluencemaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeErosionmaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeDepositionmaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeRemovalmaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeEvaporationmaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeWeatheringmaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeCutanglemaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeReposeanglemaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldErode {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldErode {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain to Erode"
    pub fn set_input_terrain_to_erode<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain to Erode" and specifies the output index of the target node.
    pub fn set_input_terrain_to_erode_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Masks"
    pub fn set_input_masks<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Masks" and specifies the output index of the target node.
    pub fn set_input_masks_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_resimulate(mut self) -> Self {
        self.params.insert("resimulate".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_erosionscale(mut self, val: f32) -> Self {
        self.params.insert("erosionscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_erosionscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("erosionscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erodability(mut self, val: f32) -> Self {
        self.params.insert("erodability".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_erodability_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodability".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flow(mut self, val: f32) -> Self {
        self.params.insert("flow".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_flow_expr(mut self, expr: &str) -> Self {
        self.params.insert("flow".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bankangle(mut self, val: f32) -> Self {
        self.params.insert("bankangle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bankangle_expr(mut self, expr: &str) -> Self {
        self.params.insert("bankangle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_coverage(mut self, val: f32) -> Self {
        self.params.insert("coverage".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_coverage_expr(mut self, expr: &str) -> Self {
        self.params.insert("coverage".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_slopeinfluence(mut self, val: f32) -> Self {
        self.params.insert("slopeinfluence".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_slopeinfluence_expr(mut self, expr: &str) -> Self {
        self.params.insert("slopeinfluence".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erosion(mut self, val: f32) -> Self {
        self.params.insert("erosion".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_erosion_expr(mut self, expr: &str) -> Self {
        self.params.insert("erosion".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_deposition(mut self, val: f32) -> Self {
        self.params.insert("deposition".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_deposition_expr(mut self, expr: &str) -> Self {
        self.params.insert("deposition".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_removal(mut self, val: f32) -> Self {
        self.params.insert("removal".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_removal_expr(mut self, expr: &str) -> Self {
        self.params.insert("removal".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_evaporation(mut self, val: f32) -> Self {
        self.params.insert("evaporation".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_evaporation_expr(mut self, expr: &str) -> Self {
        self.params.insert("evaporation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_weathering(mut self, val: f32) -> Self {
        self.params.insert("weathering".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_weathering_expr(mut self, expr: &str) -> Self {
        self.params.insert("weathering".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cutangle(mut self, val: f32) -> Self {
        self.params.insert("cutangle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_cutangle_expr(mut self, expr: &str) -> Self {
        self.params.insert("cutangle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_reposeangle(mut self, val: f32) -> Self {
        self.params.insert("reposeangle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_reposeangle_expr(mut self, expr: &str) -> Self {
        self.params.insert("reposeangle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_freezeframe(mut self, val: i32) -> Self {
        self.params.insert("freezeframe".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_freezeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("freezeframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_spreaditers(mut self, val: i32) -> Self {
        self.params.insert("spreaditers".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_spreaditers_expr(mut self, expr: &str) -> Self {
        self.params.insert("spreaditers".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_startframe(mut self, val: i32) -> Self {
        self.params.insert("startframe".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_startframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("startframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cachedframes(mut self, val: i32) -> Self {
        self.params.insert("cachedframes".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_cachedframes_expr(mut self, expr: &str) -> Self {
        self.params.insert("cachedframes".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_checkpointframes(mut self, val: i32) -> Self {
        self.params.insert("checkpointframes".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_checkpointframes_expr(mut self, expr: &str) -> Self {
        self.params.insert("checkpointframes".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_erodabilitymaskmode(mut self, val: SopHeightfieldErodeErodabilitymaskmode) -> Self {
        self.params.insert("erodabilitymaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_erodabilitymaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodabilitymaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flowmaskmode(mut self, val: SopHeightfieldErodeFlowmaskmode) -> Self {
        self.params.insert("flowmaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_flowmaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("flowmaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bankanglemaskmode(mut self, val: SopHeightfieldErodeBankanglemaskmode) -> Self {
        self.params.insert("bankanglemaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_bankanglemaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("bankanglemaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_coveragemaskmode(mut self, val: SopHeightfieldErodeCoveragemaskmode) -> Self {
        self.params.insert("coveragemaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_coveragemaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("coveragemaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_slopeinfluencemaskmode(mut self, val: SopHeightfieldErodeSlopeinfluencemaskmode) -> Self {
        self.params.insert("slopeinfluencemaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_slopeinfluencemaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("slopeinfluencemaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erosionmaskmode(mut self, val: SopHeightfieldErodeErosionmaskmode) -> Self {
        self.params.insert("erosionmaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_erosionmaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("erosionmaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_depositionmaskmode(mut self, val: SopHeightfieldErodeDepositionmaskmode) -> Self {
        self.params.insert("depositionmaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_depositionmaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("depositionmaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_removalmaskmode(mut self, val: SopHeightfieldErodeRemovalmaskmode) -> Self {
        self.params.insert("removalmaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_removalmaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("removalmaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_evaporationmaskmode(mut self, val: SopHeightfieldErodeEvaporationmaskmode) -> Self {
        self.params.insert("evaporationmaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_evaporationmaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("evaporationmaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_weatheringmaskmode(mut self, val: SopHeightfieldErodeWeatheringmaskmode) -> Self {
        self.params.insert("weatheringmaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_weatheringmaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("weatheringmaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cutanglemaskmode(mut self, val: SopHeightfieldErodeCutanglemaskmode) -> Self {
        self.params.insert("cutanglemaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_cutanglemaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("cutanglemaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_reposeanglemaskmode(mut self, val: SopHeightfieldErodeReposeanglemaskmode) -> Self {
        self.params.insert("reposeanglemaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_reposeanglemaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("reposeanglemaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_erodabilitymasklayer(mut self, val: &str) -> Self {
        self.params.insert("erodabilitymasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_erodabilitymasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodabilitymasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flowmasklayer(mut self, val: &str) -> Self {
        self.params.insert("flowmasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_flowmasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("flowmasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bankanglemasklayer(mut self, val: &str) -> Self {
        self.params.insert("bankanglemasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_bankanglemasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("bankanglemasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_coveragemasklayer(mut self, val: &str) -> Self {
        self.params.insert("coveragemasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_coveragemasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("coveragemasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_slopeinfluencemasklayer(mut self, val: &str) -> Self {
        self.params.insert("slopeinfluencemasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_slopeinfluencemasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("slopeinfluencemasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erosionmasklayer(mut self, val: &str) -> Self {
        self.params.insert("erosionmasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_erosionmasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("erosionmasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_depositionmasklayer(mut self, val: &str) -> Self {
        self.params.insert("depositionmasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_depositionmasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("depositionmasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_removalmasklayer(mut self, val: &str) -> Self {
        self.params.insert("removalmasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_removalmasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("removalmasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_evaporationmasklayer(mut self, val: &str) -> Self {
        self.params.insert("evaporationmasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_evaporationmasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("evaporationmasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_weatheringmasklayer(mut self, val: &str) -> Self {
        self.params.insert("weatheringmasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_weatheringmasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("weatheringmasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cutanglemasklayer(mut self, val: &str) -> Self {
        self.params.insert("cutanglemasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_cutanglemasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("cutanglemasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_reposeanglemasklayer(mut self, val: &str) -> Self {
        self.params.insert("reposeanglemasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_reposeanglemasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("reposeanglemasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightlayer(mut self, val: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_heightlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_debrislayer(mut self, val: &str) -> Self {
        self.params.insert("debrislayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_debrislayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("debrislayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sedimentlayer(mut self, val: &str) -> Self {
        self.params.insert("sedimentlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sedimentlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("sedimentlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flowlayer(mut self, val: &str) -> Self {
        self.params.insert("flowlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_flowlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("flowlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flowdirlayer(mut self, val: &str) -> Self {
        self.params.insert("flowdirlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_flowdirlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("flowdirlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_dofreeze(mut self, val: bool) -> Self {
        self.params.insert("dofreeze".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dofreeze_expr(mut self, expr: &str) -> Self {
        self.params.insert("dofreeze".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_resetdebrislayer(mut self, val: bool) -> Self {
        self.params.insert("resetdebrislayer".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_resetdebrislayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("resetdebrislayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_resetsedimentlayer(mut self, val: bool) -> Self {
        self.params.insert("resetsedimentlayer".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_resetsedimentlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("resetsedimentlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_resetflowlayer(mut self, val: bool) -> Self {
        self.params.insert("resetflowlayer".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_resetflowlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("resetflowlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_resetflowdirlayer(mut self, val: bool) -> Self {
        self.params.insert("resetflowdirlayer".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_resetflowdirlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("resetflowdirlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_adddebris(mut self, val: bool) -> Self {
        self.params.insert("adddebris".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_adddebris_expr(mut self, expr: &str) -> Self {
        self.params.insert("adddebris".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_addsediment(mut self, val: bool) -> Self {
        self.params.insert("addsediment".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_addsediment_expr(mut self, expr: &str) -> Self {
        self.params.insert("addsediment".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cacheenabled(mut self, val: bool) -> Self {
        self.params.insert("cacheenabled".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_cacheenabled_expr(mut self, expr: &str) -> Self {
        self.params.insert("cacheenabled".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldErode {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_erode"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeHydroErodabilitymaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeHydroBankanglemaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeHydroRemovalratemaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeHydroGridbiasmaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeHydroBedDepositionratemaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldErodeHydro {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldErodeHydro {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2"
    pub fn set_input_sub_network_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_2_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_globalerosionrate(mut self, val: f32) -> Self {
        self.params.insert("globalerosionrate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_globalerosionrate_expr(mut self, expr: &str) -> Self {
        self.params.insert("globalerosionrate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erodability(mut self, val: f32) -> Self {
        self.params.insert("erodability".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_erodability_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodability".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erosionrate(mut self, val: f32) -> Self {
        self.params.insert("erosionrate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_erosionrate_expr(mut self, expr: &str) -> Self {
        self.params.insert("erosionrate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bankangle(mut self, val: f32) -> Self {
        self.params.insert("bankangle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bankangle_expr(mut self, expr: &str) -> Self {
        self.params.insert("bankangle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_removalrate(mut self, val: f32) -> Self {
        self.params.insert("removalrate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_removalrate_expr(mut self, expr: &str) -> Self {
        self.params.insert("removalrate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxdebrisdepth(mut self, val: f32) -> Self {
        self.params.insert("maxdebrisdepth".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maxdebrisdepth_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxdebrisdepth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridbias(mut self, val: f32) -> Self {
        self.params.insert("gridbias".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gridbias_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridbias".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erodability_initialfactor(mut self, val: f32) -> Self {
        self.params.insert("erodability_initialfactor".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_erodability_initialfactor_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodability_initialfactor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erodability_slopefactor(mut self, val: f32) -> Self {
        self.params.insert("erodability_slopefactor".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_erodability_slopefactor_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodability_slopefactor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bed_erosionratefactor(mut self, val: f32) -> Self {
        self.params.insert("bed_erosionratefactor".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bed_erosionratefactor_expr(mut self, expr: &str) -> Self {
        self.params.insert("bed_erosionratefactor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bed_depositionrate(mut self, val: f32) -> Self {
        self.params.insert("bed_depositionrate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bed_depositionrate_expr(mut self, expr: &str) -> Self {
        self.params.insert("bed_depositionrate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bed_sedimentcap(mut self, val: f32) -> Self {
        self.params.insert("bed_sedimentcap".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bed_sedimentcap_expr(mut self, expr: &str) -> Self {
        self.params.insert("bed_sedimentcap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank_erosionratefactor(mut self, val: f32) -> Self {
        self.params.insert("bank_erosionratefactor".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_erosionratefactor_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank_erosionratefactor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank_maxbankbedwaterratio(mut self, val: f32) -> Self {
        self.params.insert("bank_maxbankbedwaterratio".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_maxbankbedwaterratio_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank_maxbankbedwaterratio".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_quant(mut self, val: f32) -> Self {
        self.params.insert("quant".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_quant_expr(mut self, expr: &str) -> Self {
        self.params.insert("quant".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strata_depth(mut self, val: f32) -> Self {
        self.params.insert("strata_depth".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_strata_depth_expr(mut self, expr: &str) -> Self {
        self.params.insert("strata_depth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_erodability_rampupiters(mut self, val: i32) -> Self {
        self.params.insert("erodability_rampupiters".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_erodability_rampupiters_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodability_rampupiters".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_erodabilitymaskmode(mut self, val: SopHeightfieldErodeHydroErodabilitymaskmode) -> Self {
        self.params.insert("erodabilitymaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_erodabilitymaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodabilitymaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bankanglemaskmode(mut self, val: SopHeightfieldErodeHydroBankanglemaskmode) -> Self {
        self.params.insert("bankanglemaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_bankanglemaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("bankanglemaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_removalratemaskmode(mut self, val: SopHeightfieldErodeHydroRemovalratemaskmode) -> Self {
        self.params.insert("removalratemaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_removalratemaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("removalratemaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridbiasmaskmode(mut self, val: SopHeightfieldErodeHydroGridbiasmaskmode) -> Self {
        self.params.insert("gridbiasmaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_gridbiasmaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridbiasmaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bed_depositionratemaskmode(mut self, val: SopHeightfieldErodeHydroBedDepositionratemaskmode) -> Self {
        self.params.insert("bed_depositionratemaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_bed_depositionratemaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("bed_depositionratemaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_strata_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("strata_ramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_strata_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("strata_ramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_erodabilitymasklayer(mut self, val: &str) -> Self {
        self.params.insert("erodabilitymasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_erodabilitymasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodabilitymasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bankanglemasklayer(mut self, val: &str) -> Self {
        self.params.insert("bankanglemasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_bankanglemasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("bankanglemasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_removalratemasklayer(mut self, val: &str) -> Self {
        self.params.insert("removalratemasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_removalratemasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("removalratemasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridbiasmasklayer(mut self, val: &str) -> Self {
        self.params.insert("gridbiasmasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_gridbiasmasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridbiasmasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bed_depositionratemasklayer(mut self, val: &str) -> Self {
        self.params.insert("bed_depositionratemasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_bed_depositionratemasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("bed_depositionratemasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_slumplayer(mut self, val: &str) -> Self {
        self.params.insert("slumplayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_slumplayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("slumplayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_debrislayer(mut self, val: &str) -> Self {
        self.params.insert("debrislayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_debrislayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("debrislayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sedimentlayer(mut self, val: &str) -> Self {
        self.params.insert("sedimentlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sedimentlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("sedimentlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erodinglayer(mut self, val: &str) -> Self {
        self.params.insert("erodinglayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_erodinglayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodinglayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_belowlayers(mut self, val: &str) -> Self {
        self.params.insert("belowlayers".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_belowlayers_expr(mut self, expr: &str) -> Self {
        self.params.insert("belowlayers".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bedrocklayer(mut self, val: &str) -> Self {
        self.params.insert("bedrocklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_bedrocklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("bedrocklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_dostrata(mut self, val: bool) -> Self {
        self.params.insert("dostrata".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dostrata_expr(mut self, expr: &str) -> Self {
        self.params.insert("dostrata".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strata_clamp(mut self, val: bool) -> Self {
        self.params.insert("strata_clamp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_strata_clamp_expr(mut self, expr: &str) -> Self {
        self.params.insert("strata_clamp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldErodeHydro {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_erode_hydro"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodePrecipitationRainAmountmaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldErodePrecipitation {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldErodePrecipitation {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Water Layer"
    pub fn set_input_water_layer<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Water Layer" and specifies the output index of the target node.
    pub fn set_input_water_layer_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Precipitation Mask"
    pub fn set_input_precipitation_mask<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Precipitation Mask" and specifies the output index of the target node.
    pub fn set_input_precipitation_mask_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_rain_amount(mut self, val: f32) -> Self {
        self.params.insert("rain_amount".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rain_amount_expr(mut self, expr: &str) -> Self {
        self.params.insert("rain_amount".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rain_density(mut self, val: f32) -> Self {
        self.params.insert("rain_density".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rain_density_expr(mut self, expr: &str) -> Self {
        self.params.insert("rain_density".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rain_expandradius(mut self, val: f32) -> Self {
        self.params.insert("rain_expandradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rain_expandradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("rain_expandradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rain_blurradius(mut self, val: f32) -> Self {
        self.params.insert("rain_blurradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rain_blurradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("rain_blurradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_rain_amountmaskmode(mut self, val: SopHeightfieldErodePrecipitationRainAmountmaskmode) -> Self {
        self.params.insert("rain_amountmaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rain_amountmaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("rain_amountmaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_rain_amountmasklayer(mut self, val: &str) -> Self {
        self.params.insert("rain_amountmasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_rain_amountmasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("rain_amountmasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightlayer(mut self, val: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_heightlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_waterlayer(mut self, val: &str) -> Self {
        self.params.insert("waterlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_waterlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("waterlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldErodePrecipitation {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_erode_precipitation"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeThermalErodabilitymaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeThermalCutanglemaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeThermalRemovalratemaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeThermalGridbiasmaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldErodeThermalReposeanglemaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldErodeThermal {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldErodeThermal {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask"
    pub fn set_input_mask<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask" and specifies the output index of the target node.
    pub fn set_input_mask_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_globalerosionrate(mut self, val: f32) -> Self {
        self.params.insert("globalerosionrate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_globalerosionrate_expr(mut self, expr: &str) -> Self {
        self.params.insert("globalerosionrate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erodability(mut self, val: f32) -> Self {
        self.params.insert("erodability".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_erodability_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodability".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erosionrate(mut self, val: f32) -> Self {
        self.params.insert("erosionrate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_erosionrate_expr(mut self, expr: &str) -> Self {
        self.params.insert("erosionrate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cutangle(mut self, val: f32) -> Self {
        self.params.insert("cutangle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_cutangle_expr(mut self, expr: &str) -> Self {
        self.params.insert("cutangle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_removalrate(mut self, val: f32) -> Self {
        self.params.insert("removalrate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_removalrate_expr(mut self, expr: &str) -> Self {
        self.params.insert("removalrate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxdebrisdepth(mut self, val: f32) -> Self {
        self.params.insert("maxdebrisdepth".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maxdebrisdepth_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxdebrisdepth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridbias(mut self, val: f32) -> Self {
        self.params.insert("gridbias".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gridbias_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridbias".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_quant(mut self, val: f32) -> Self {
        self.params.insert("quant".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_quant_expr(mut self, expr: &str) -> Self {
        self.params.insert("quant".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_reposeangle(mut self, val: f32) -> Self {
        self.params.insert("reposeangle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_reposeangle_expr(mut self, expr: &str) -> Self {
        self.params.insert("reposeangle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strata_depth(mut self, val: f32) -> Self {
        self.params.insert("strata_depth".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_strata_depth_expr(mut self, expr: &str) -> Self {
        self.params.insert("strata_depth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_erodabilitymaskmode(mut self, val: SopHeightfieldErodeThermalErodabilitymaskmode) -> Self {
        self.params.insert("erodabilitymaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_erodabilitymaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodabilitymaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cutanglemaskmode(mut self, val: SopHeightfieldErodeThermalCutanglemaskmode) -> Self {
        self.params.insert("cutanglemaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_cutanglemaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("cutanglemaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_removalratemaskmode(mut self, val: SopHeightfieldErodeThermalRemovalratemaskmode) -> Self {
        self.params.insert("removalratemaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_removalratemaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("removalratemaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridbiasmaskmode(mut self, val: SopHeightfieldErodeThermalGridbiasmaskmode) -> Self {
        self.params.insert("gridbiasmaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_gridbiasmaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridbiasmaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_reposeanglemaskmode(mut self, val: SopHeightfieldErodeThermalReposeanglemaskmode) -> Self {
        self.params.insert("reposeanglemaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_reposeanglemaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("reposeanglemaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_strata_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("strata_ramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_strata_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("strata_ramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_erodabilitymasklayer(mut self, val: &str) -> Self {
        self.params.insert("erodabilitymasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_erodabilitymasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodabilitymasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cutanglemasklayer(mut self, val: &str) -> Self {
        self.params.insert("cutanglemasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_cutanglemasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("cutanglemasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_removalratemasklayer(mut self, val: &str) -> Self {
        self.params.insert("removalratemasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_removalratemasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("removalratemasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridbiasmasklayer(mut self, val: &str) -> Self {
        self.params.insert("gridbiasmasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_gridbiasmasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridbiasmasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_reposeanglemasklayer(mut self, val: &str) -> Self {
        self.params.insert("reposeanglemasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_reposeanglemasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("reposeanglemasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stabilitymasklayer(mut self, val: &str) -> Self {
        self.params.insert("stabilitymasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_stabilitymasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("stabilitymasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_materiallayer(mut self, val: &str) -> Self {
        self.params.insert("materiallayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_materiallayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("materiallayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erodinglayer(mut self, val: &str) -> Self {
        self.params.insert("erodinglayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_erodinglayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodinglayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_belowlayers(mut self, val: &str) -> Self {
        self.params.insert("belowlayers".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_belowlayers_expr(mut self, expr: &str) -> Self {
        self.params.insert("belowlayers".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bedrocklayer(mut self, val: &str) -> Self {
        self.params.insert("bedrocklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_bedrocklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("bedrocklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_dostrata(mut self, val: bool) -> Self {
        self.params.insert("dostrata".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dostrata_expr(mut self, expr: &str) -> Self {
        self.params.insert("dostrata".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strata_clamp(mut self, val: bool) -> Self {
        self.params.insert("strata_clamp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_strata_clamp_expr(mut self, expr: &str) -> Self {
        self.params.insert("strata_clamp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldErodeThermal {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_erode_thermal"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldFileSource {
    File = 0,
    Cop = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldFileMonoop {
    Luminance = 0,
    NtscLuminance = 1,
    Average = 2,
    MaximumComponent = 3,
    MinimumComponent = 4,
    Magnitude = 5,
    Hue = 6,
    Saturation = 7,
    Red = 8,
    Green = 9,
    Blue = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldFileType {
    HeightField = 0,
    Mask = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldFileLayermode {
    Replace = 0,
    Add = 1,
    Subtract = 2,
    Difference = 3,
    Multiply = 4,
    Maximum = 5,
    Minimum = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldFileLayerborder {
    Constant = 0,
    Repeat = 1,
    Streak = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldFileSizemethod {
    SizeOfLargestAxis = 0,
    GridSpacing = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldFileOrient {
    Xy = 0,
    Yz = 1,
    Zx = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldFileSampling {
    Center = 0,
    Corner = 1,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldFile {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldFile {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain to Load Into"
    pub fn set_input_terrain_to_load_into<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain to Load Into" and specifies the output index of the target node.
    pub fn set_input_terrain_to_load_into_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_reload(mut self) -> Self {
        self.params.insert("reload".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_layerborderval(mut self, val: f32) -> Self {
        self.params.insert("layerborderval".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_layerborderval_expr(mut self, expr: &str) -> Self {
        self.params.insert("layerborderval".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_size(mut self, val: f32) -> Self {
        self.params.insert("size".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert("size".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridspacing(mut self, val: f32) -> Self {
        self.params.insert("gridspacing".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gridspacing_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridspacing".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uniformscale(mut self, val: f32) -> Self {
        self.params.insert("uniformscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_uniformscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("uniformscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightscale(mut self, val: f32) -> Self {
        self.params.insert("heightscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_heightscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clampmin(mut self, val: f32) -> Self {
        self.params.insert("clampmin".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_clampmin_expr(mut self, expr: &str) -> Self {
        self.params.insert("clampmin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clampmax(mut self, val: f32) -> Self {
        self.params.insert("clampmax".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_clampmax_expr(mut self, expr: &str) -> Self {
        self.params.insert("clampmax".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rotate(mut self, val: f32) -> Self {
        self.params.insert("rotate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert("rotate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_t(mut self, val: [f32; 2]) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_frame(mut self, val: i32) -> Self {
        self.params.insert("frame".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_frame_expr(mut self, expr: &str) -> Self {
        self.params.insert("frame".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_source(mut self, val: SopHeightfieldFileSource) -> Self {
        self.params.insert("source".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_source_expr(mut self, expr: &str) -> Self {
        self.params.insert("source".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_monoop(mut self, val: SopHeightfieldFileMonoop) -> Self {
        self.params.insert("monoop".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_monoop_expr(mut self, expr: &str) -> Self {
        self.params.insert("monoop".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_type(mut self, val: SopHeightfieldFileType) -> Self {
        self.params.insert("type".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert("type".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_layermode(mut self, val: SopHeightfieldFileLayermode) -> Self {
        self.params.insert("layermode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_layermode_expr(mut self, expr: &str) -> Self {
        self.params.insert("layermode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_layerborder(mut self, val: SopHeightfieldFileLayerborder) -> Self {
        self.params.insert("layerborder".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_layerborder_expr(mut self, expr: &str) -> Self {
        self.params.insert("layerborder".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizemethod(mut self, val: SopHeightfieldFileSizemethod) -> Self {
        self.params.insert("sizemethod".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_sizemethod_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizemethod".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_orient(mut self, val: SopHeightfieldFileOrient) -> Self {
        self.params.insert("orient".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_orient_expr(mut self, expr: &str) -> Self {
        self.params.insert("orient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sampling(mut self, val: SopHeightfieldFileSampling) -> Self {
        self.params.insert("sampling".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_sampling_expr(mut self, expr: &str) -> Self {
        self.params.insert("sampling".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_coppath(mut self, val: &str) -> Self {
        self.params.insert("coppath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_coppath_expr(mut self, expr: &str) -> Self {
        self.params.insert("coppath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_filename(mut self, val: &str) -> Self {
        self.params.insert("filename".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_filename_expr(mut self, expr: &str) -> Self {
        self.params.insert("filename".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_layer(mut self, val: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_layer_expr(mut self, expr: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_useframe(mut self, val: bool) -> Self {
        self.params.insert("useframe".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("useframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_doclampmin(mut self, val: bool) -> Self {
        self.params.insert("doclampmin".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_doclampmin_expr(mut self, expr: &str) -> Self {
        self.params.insert("doclampmin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_doclampmax(mut self, val: bool) -> Self {
        self.params.insert("doclampmax".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_doclampmax_expr(mut self, expr: &str) -> Self {
        self.params.insert("doclampmax".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldFile {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_file"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldFlattenMethod {
    FlattenToValue = 0,
    FlattenToAverageHeight = 1,
    FlattenSlopes = 2,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldFlatten {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldFlatten {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain to Flatten"
    pub fn set_input_terrain_to_flatten<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain to Flatten" and specifies the output index of the target node.
    pub fn set_input_terrain_to_flatten_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_blurradius(mut self, val: f32) -> Self {
        self.params.insert("blurradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blurradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("blurradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_height(mut self, val: f32) -> Self {
        self.params.insert("height".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_height_expr(mut self, expr: &str) -> Self {
        self.params.insert("height".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: SopHeightfieldFlattenMethod) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_masklayer(mut self, val: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_masklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightlayer(mut self, val: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_heightlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldFlatten {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_flatten"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldFlowfieldSlumpmode {
    Smooth = 0,
    Granular = 1,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldFlowfield {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldFlowfield {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_rainamount(mut self, val: f32) -> Self {
        self.params.insert("rainamount".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rainamount_expr(mut self, expr: &str) -> Self {
        self.params.insert("rainamount".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_raindensity(mut self, val: f32) -> Self {
        self.params.insert("raindensity".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_raindensity_expr(mut self, expr: &str) -> Self {
        self.params.insert("raindensity".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maskscale(mut self, val: f32) -> Self {
        self.params.insert("maskscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maskscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightscale(mut self, val: f32) -> Self {
        self.params.insert("heightscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_heightscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_smoothiterations(mut self, val: i32) -> Self {
        self.params.insert("smoothiterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_smoothiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("smoothiterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_slumpmode(mut self, val: SopHeightfieldFlowfieldSlumpmode) -> Self {
        self.params.insert("slumpmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_slumpmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("slumpmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_heightlayer(mut self, val: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_heightlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_waterlayer(mut self, val: &str) -> Self {
        self.params.insert("waterlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_waterlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("waterlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flowlayer(mut self, val: &str) -> Self {
        self.params.insert("flowlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_flowlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("flowlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flowdirlayer(mut self, val: &str) -> Self {
        self.params.insert("flowdirlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_flowdirlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("flowdirlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_domask(mut self, val: bool) -> Self {
        self.params.insert("domask".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_domask_expr(mut self, expr: &str) -> Self {
        self.params.insert("domask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_doheight(mut self, val: bool) -> Self {
        self.params.insert("doheight".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_doheight_expr(mut self, expr: &str) -> Self {
        self.params.insert("doheight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldFlowfield {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_flowfield"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone)]
pub struct SopHeightfieldIsolatelayer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldIsolatelayer {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- String parameters ---
    pub fn with_layer(mut self, val: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_layer_expr(mut self, expr: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_overwriteheight(mut self, val: bool) -> Self {
        self.params.insert("overwriteheight".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_overwriteheight_expr(mut self, expr: &str) -> Self {
        self.params.insert("overwriteheight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_overwritemask(mut self, val: bool) -> Self {
        self.params.insert("overwritemask".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_overwritemask_expr(mut self, expr: &str) -> Self {
        self.params.insert("overwritemask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldIsolatelayer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_isolatelayer"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldLayerMode {
    Replace = 0,
    Add = 1,
    Subtract = 2,
    Difference = 3,
    Multiply = 4,
    Maximum = 5,
    Minimum = 6,
    Blend = 7,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldLayer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldLayer {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Base Terrain"
    pub fn set_input_base_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Base Terrain" and specifies the output index of the target node.
    pub fn set_input_base_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Terrain to Layer"
    pub fn set_input_terrain_to_layer<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Terrain to Layer" and specifies the output index of the target node.
    pub fn set_input_terrain_to_layer_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Mask"
    pub fn set_input_mask<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Mask" and specifies the output index of the target node.
    pub fn set_input_mask_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maskweight(mut self, val: f32) -> Self {
        self.params.insert("maskweight".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maskweight_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskweight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_base_offset(mut self, val: f32) -> Self {
        self.params.insert("base_offset".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_base_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("base_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_base_scale(mut self, val: f32) -> Self {
        self.params.insert("base_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_base_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("base_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_layer_offset(mut self, val: f32) -> Self {
        self.params.insert("layer_offset".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_layer_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("layer_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_layer_scale(mut self, val: f32) -> Self {
        self.params.insert("layer_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_layer_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("layer_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_final_offset(mut self, val: f32) -> Self {
        self.params.insert("final_offset".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_final_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("final_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_final_scale(mut self, val: f32) -> Self {
        self.params.insert("final_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_final_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("final_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clampmin(mut self, val: f32) -> Self {
        self.params.insert("clampmin".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_clampmin_expr(mut self, expr: &str) -> Self {
        self.params.insert("clampmin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clampmax(mut self, val: f32) -> Self {
        self.params.insert("clampmax".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_clampmax_expr(mut self, expr: &str) -> Self {
        self.params.insert("clampmax".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_mode(mut self, val: SopHeightfieldLayerMode) -> Self {
        self.params.insert("mode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert("mode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_layer(mut self, val: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_layer_expr(mut self, expr: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_masklayer(mut self, val: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_masklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_invertmask(mut self, val: bool) -> Self {
        self.params.insert("invertmask".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_invertmask_expr(mut self, expr: &str) -> Self {
        self.params.insert("invertmask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_doclampmin(mut self, val: bool) -> Self {
        self.params.insert("doclampmin".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_doclampmin_expr(mut self, expr: &str) -> Self {
        self.params.insert("doclampmin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_doclampmax(mut self, val: bool) -> Self {
        self.params.insert("doclampmax".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_doclampmax_expr(mut self, expr: &str) -> Self {
        self.params.insert("doclampmax".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldLayer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_layer"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone)]
pub struct SopHeightfieldLayerclear {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldLayerclear {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_value_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(format!("value{}", index1), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_value_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("value{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_layer_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("layer{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_layer_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("layer{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldLayerclear {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_layerclear"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldLayerpropertyBorder {
    Constant = 0,
    Repeat = 1,
    Streak = 2,
    Sdf = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldLayerpropertyCompresspreset {
    /// Initialize ↓
    Initialize = 0,
    Uncompressed = 1,
    Float16 = 2,
    Integer16 = 3,
    Integer8 = 4,
    Integer1 = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldLayerpropertyDither {
    None = 0,
    Ordered = 1,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldLayerproperty {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldLayerproperty {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_borderval(mut self, val: f32) -> Self {
        self.params.insert("borderval".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_borderval_expr(mut self, expr: &str) -> Self {
        self.params.insert("borderval".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_compressiontol(mut self, val: f32) -> Self {
        self.params.insert("compressiontol".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_compressiontol_expr(mut self, expr: &str) -> Self {
        self.params.insert("compressiontol".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_border(mut self, val: SopHeightfieldLayerpropertyBorder) -> Self {
        self.params.insert("border".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_border_expr(mut self, expr: &str) -> Self {
        self.params.insert("border".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_compresspreset(mut self, val: SopHeightfieldLayerpropertyCompresspreset) -> Self {
        self.params.insert("compresspreset".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_compresspreset_expr(mut self, expr: &str) -> Self {
        self.params.insert("compresspreset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dither(mut self, val: SopHeightfieldLayerpropertyDither) -> Self {
        self.params.insert("dither".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_dither_expr(mut self, expr: &str) -> Self {
        self.params.insert("dither".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_layer(mut self, val: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_layer_expr(mut self, expr: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_doborder(mut self, val: bool) -> Self {
        self.params.insert("doborder".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_doborder_expr(mut self, expr: &str) -> Self {
        self.params.insert("doborder".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dovoltol(mut self, val: bool) -> Self {
        self.params.insert("dovoltol".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dovoltol_expr(mut self, expr: &str) -> Self {
        self.params.insert("dovoltol".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usefp16(mut self, val: bool) -> Self {
        self.params.insert("usefp16".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usefp16_expr(mut self, expr: &str) -> Self {
        self.params.insert("usefp16".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldLayerproperty {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_layerproperty"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldMaskbyconcavityCombine {
    Replace = 0,
    Add = 1,
    Subtract = 2,
    Difference = 3,
    Multiply = 4,
    Maximum = 5,
    Minimum = 6,
    Blend = 7,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldMaskbyconcavity {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldMaskbyconcavity {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain to Create Mask on"
    pub fn set_input_terrain_to_create_mask_on<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain to Create Mask on" and specifies the output index of the target node.
    pub fn set_input_terrain_to_create_mask_on_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_minconcavity(mut self, val: f32) -> Self {
        self.params.insert("minconcavity".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_minconcavity_expr(mut self, expr: &str) -> Self {
        self.params.insert("minconcavity".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxconcavity(mut self, val: f32) -> Self {
        self.params.insert("maxconcavity".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maxconcavity_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxconcavity".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_viewdistance(mut self, val: f32) -> Self {
        self.params.insert("viewdistance".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_viewdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert("viewdistance".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stepscale(mut self, val: f32) -> Self {
        self.params.insert("stepscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stepscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("stepscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_numsearches(mut self, val: i32) -> Self {
        self.params.insert("numsearches".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_numsearches_expr(mut self, expr: &str) -> Self {
        self.params.insert("numsearches".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_combine(mut self, val: SopHeightfieldMaskbyconcavityCombine) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_combine_expr(mut self, expr: &str) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_masklayer(mut self, val: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_masklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightlayer(mut self, val: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_heightlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_invertmask(mut self, val: bool) -> Self {
        self.params.insert("invertmask".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_invertmask_expr(mut self, expr: &str) -> Self {
        self.params.insert("invertmask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldMaskbyconcavity {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_maskbyconcavity"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldMaskbyfeatureCombine {
    Replace = 0,
    Add = 1,
    Subtract = 2,
    Difference = 3,
    Multiply = 4,
    Maximum = 5,
    Minimum = 6,
    Blend = 7,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldMaskbyfeature {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldMaskbyfeature {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain to Create Mask on"
    pub fn set_input_terrain_to_create_mask_on<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain to Create Mask on" and specifies the output index of the target node.
    pub fn set_input_terrain_to_create_mask_on_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_computerange(mut self) -> Self {
        self.params.insert("computerange".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_computercurvature(mut self) -> Self {
        self.params.insert("computercurvature".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_smooth_radius(mut self, val: f32) -> Self {
        self.params.insert("smooth_radius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_smooth_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert("smooth_radius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_min_slopeangle(mut self, val: f32) -> Self {
        self.params.insert("min_slopeangle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_min_slopeangle_expr(mut self, expr: &str) -> Self {
        self.params.insert("min_slopeangle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_max_slopeangle(mut self, val: f32) -> Self {
        self.params.insert("max_slopeangle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_max_slopeangle_expr(mut self, expr: &str) -> Self {
        self.params.insert("max_slopeangle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_minheight(mut self, val: f32) -> Self {
        self.params.insert("minheight".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_minheight_expr(mut self, expr: &str) -> Self {
        self.params.insert("minheight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxheight(mut self, val: f32) -> Self {
        self.params.insert("maxheight".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maxheight_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxheight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_max_curvature(mut self, val: f32) -> Self {
        self.params.insert("max_curvature".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_max_curvature_expr(mut self, expr: &str) -> Self {
        self.params.insert("max_curvature".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_goal_angle(mut self, val: f32) -> Self {
        self.params.insert("goal_angle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_goal_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert("goal_angle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_angle_spread(mut self, val: f32) -> Self {
        self.params.insert("angle_spread".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_angle_spread_expr(mut self, expr: &str) -> Self {
        self.params.insert("angle_spread".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_minexposure(mut self, val: f32) -> Self {
        self.params.insert("minexposure".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_minexposure_expr(mut self, expr: &str) -> Self {
        self.params.insert("minexposure".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxexposure(mut self, val: f32) -> Self {
        self.params.insert("maxexposure".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maxexposure_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxexposure".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_viewdistance(mut self, val: f32) -> Self {
        self.params.insert("viewdistance".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_viewdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert("viewdistance".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stepscale(mut self, val: f32) -> Self {
        self.params.insert("stepscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stepscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("stepscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_numsearches(mut self, val: i32) -> Self {
        self.params.insert("numsearches".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_numsearches_expr(mut self, expr: &str) -> Self {
        self.params.insert("numsearches".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_combine(mut self, val: SopHeightfieldMaskbyfeatureCombine) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_combine_expr(mut self, expr: &str) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_sloperamp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("sloperamp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_sloperamp_expr(mut self, expr: &str) -> Self {
        self.params.insert("sloperamp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("heightramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_heightramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_curvatureramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("curvatureramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_curvatureramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("curvatureramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dirramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("dirramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_dirramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("dirramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("ramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("ramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_heightlayer(mut self, val: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_heightlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_masklayer(mut self, val: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_masklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_invertmask(mut self, val: bool) -> Self {
        self.params.insert("invertmask".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_invertmask_expr(mut self, expr: &str) -> Self {
        self.params.insert("invertmask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maskbyslope(mut self, val: bool) -> Self {
        self.params.insert("maskbyslope".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_maskbyslope_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskbyslope".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maskbyheight(mut self, val: bool) -> Self {
        self.params.insert("maskbyheight".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_maskbyheight_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskbyheight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maskbycurvature(mut self, val: bool) -> Self {
        self.params.insert("maskbycurvature".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_maskbycurvature_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskbycurvature".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maskbydir(mut self, val: bool) -> Self {
        self.params.insert("maskbydir".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_maskbydir_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskbydir".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maskbyocclusion(mut self, val: bool) -> Self {
        self.params.insert("maskbyocclusion".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_maskbyocclusion_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskbyocclusion".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldMaskbyfeature {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_maskbyfeature"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldMaskbyobjectCombine {
    Replace = 0,
    Add = 1,
    Subtract = 2,
    Difference = 3,
    Multiply = 4,
    Maximum = 5,
    Minimum = 6,
    Blend = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldMaskbyobjectMethod {
    Project = 0,
    FogVolume = 1,
    SdfVolume = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldMaskbyobjectMaskdir {
    EitherSide = 0,
    AboveHeightfield = 1,
    BelowHeightfield = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldMaskbyobjectBlurmethod {
    Blur = 0,
    BoxBlur = 1,
    Expand = 2,
    Shrink = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldMaskbyobjectJittercombine {
    Average = 0,
    Median = 1,
    Shortest = 2,
    Longest = 3,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldMaskbyobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldMaskbyobject {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain to Mask"
    pub fn set_input_terrain_to_mask<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain to Mask" and specifies the output index of the target node.
    pub fn set_input_terrain_to_mask_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Geometry to Build Mask From"
    pub fn set_input_geometry_to_build_mask_from<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Geometry to Build Mask From" and specifies the output index of the target node.
    pub fn set_input_geometry_to_build_mask_from_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxdist(mut self, val: f32) -> Self {
        self.params.insert("maxdist".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxdist".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_value(mut self, val: f32) -> Self {
        self.params.insert("value".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_value_expr(mut self, expr: &str) -> Self {
        self.params.insert("value".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blurradius(mut self, val: f32) -> Self {
        self.params.insert("blurradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blurradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("blurradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_jitter(mut self, val: f32) -> Self {
        self.params.insert("jitter".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_jitter_expr(mut self, expr: &str) -> Self {
        self.params.insert("jitter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_sample(mut self, val: i32) -> Self {
        self.params.insert("sample".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_sample_expr(mut self, expr: &str) -> Self {
        self.params.insert("sample".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_combine(mut self, val: SopHeightfieldMaskbyobjectCombine) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_combine_expr(mut self, expr: &str) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_method(mut self, val: SopHeightfieldMaskbyobjectMethod) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maskdir(mut self, val: SopHeightfieldMaskbyobjectMaskdir) -> Self {
        self.params.insert("maskdir".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_maskdir_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskdir".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blurmethod(mut self, val: SopHeightfieldMaskbyobjectBlurmethod) -> Self {
        self.params.insert("blurmethod".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_blurmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert("blurmethod".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_jittercombine(mut self, val: SopHeightfieldMaskbyobjectJittercombine) -> Self {
        self.params.insert("jittercombine".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_jittercombine_expr(mut self, expr: &str) -> Self {
        self.params.insert("jittercombine".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_masklayer(mut self, val: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_masklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightlayer(mut self, val: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_heightlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_invertmask(mut self, val: bool) -> Self {
        self.params.insert("invertmask".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_invertmask_expr(mut self, expr: &str) -> Self {
        self.params.insert("invertmask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dojitter(mut self, val: bool) -> Self {
        self.params.insert("dojitter".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dojitter_expr(mut self, expr: &str) -> Self {
        self.params.insert("dojitter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldMaskbyobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_maskbyobject"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldMaskbyocclusionCombine {
    Replace = 0,
    Add = 1,
    Subtract = 2,
    Difference = 3,
    Multiply = 4,
    Maximum = 5,
    Minimum = 6,
    Blend = 7,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldMaskbyocclusion {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldMaskbyocclusion {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain to Create Mask on"
    pub fn set_input_terrain_to_create_mask_on<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain to Create Mask on" and specifies the output index of the target node.
    pub fn set_input_terrain_to_create_mask_on_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_minexposure(mut self, val: f32) -> Self {
        self.params.insert("minexposure".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_minexposure_expr(mut self, expr: &str) -> Self {
        self.params.insert("minexposure".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxexposure(mut self, val: f32) -> Self {
        self.params.insert("maxexposure".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maxexposure_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxexposure".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_viewdistance(mut self, val: f32) -> Self {
        self.params.insert("viewdistance".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_viewdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert("viewdistance".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stepscale(mut self, val: f32) -> Self {
        self.params.insert("stepscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stepscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("stepscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_numsearches(mut self, val: i32) -> Self {
        self.params.insert("numsearches".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_numsearches_expr(mut self, expr: &str) -> Self {
        self.params.insert("numsearches".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_combine(mut self, val: SopHeightfieldMaskbyocclusionCombine) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_combine_expr(mut self, expr: &str) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("ramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("ramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_heightlayer(mut self, val: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_heightlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_masklayer(mut self, val: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_masklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_dohemisphere(mut self, val: bool) -> Self {
        self.params.insert("dohemisphere".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dohemisphere_expr(mut self, expr: &str) -> Self {
        self.params.insert("dohemisphere".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldMaskbyocclusion {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_maskbyocclusion"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldMaskbyshadowCombine {
    Replace = 0,
    Add = 1,
    Subtract = 2,
    Difference = 3,
    Multiply = 4,
    Maximum = 5,
    Minimum = 6,
    Blend = 7,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldMaskbyshadow {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldMaskbyshadow {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain to Create Mask on"
    pub fn set_input_terrain_to_create_mask_on<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain to Create Mask on" and specifies the output index of the target node.
    pub fn set_input_terrain_to_create_mask_on_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lightdir(mut self, val: f32) -> Self {
        self.params.insert("lightdir".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_lightdir_expr(mut self, expr: &str) -> Self {
        self.params.insert("lightdir".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lightangle(mut self, val: f32) -> Self {
        self.params.insert("lightangle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_lightangle_expr(mut self, expr: &str) -> Self {
        self.params.insert("lightangle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_opacity(mut self, val: f32) -> Self {
        self.params.insert("opacity".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_opacity_expr(mut self, expr: &str) -> Self {
        self.params.insert("opacity".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_falloff(mut self, val: f32) -> Self {
        self.params.insert("falloff".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_falloff_expr(mut self, expr: &str) -> Self {
        self.params.insert("falloff".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_combine(mut self, val: SopHeightfieldMaskbyshadowCombine) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_combine_expr(mut self, expr: &str) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_masklayer(mut self, val: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_masklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightlayer(mut self, val: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_heightlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_invertmask(mut self, val: bool) -> Self {
        self.params.insert("invertmask".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_invertmask_expr(mut self, expr: &str) -> Self {
        self.params.insert("invertmask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldMaskbyshadow {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_maskbyshadow"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldNoiseCombine {
    Replace = 0,
    Add = 1,
    Subtract = 2,
    Difference = 3,
    Multiply = 4,
    Maximum = 5,
    Minimum = 6,
    Blend = 7,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldNoise {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldNoise {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask"
    pub fn set_input_mask<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask" and specifies the output index of the target node.
    pub fn set_input_mask_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_amp(mut self, val: f32) -> Self {
        self.params.insert("amp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert("amp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_elementsize(mut self, val: f32) -> Self {
        self.params.insert("elementsize".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_elementsize_expr(mut self, expr: &str) -> Self {
        self.params.insert("elementsize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_oct(mut self, val: f32) -> Self {
        self.params.insert("oct".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_oct_expr(mut self, expr: &str) -> Self {
        self.params.insert("oct".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lac(mut self, val: f32) -> Self {
        self.params.insert("lac".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_lac_expr(mut self, expr: &str) -> Self {
        self.params.insert("lac".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rough(mut self, val: f32) -> Self {
        self.params.insert("rough".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert("rough".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flowrot(mut self, val: f32) -> Self {
        self.params.insert("flowrot".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_flowrot_expr(mut self, expr: &str) -> Self {
        self.params.insert("flowrot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gain(mut self, val: f32) -> Self {
        self.params.insert("gain".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gain_expr(mut self, expr: &str) -> Self {
        self.params.insert("gain".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bias(mut self, val: f32) -> Self {
        self.params.insert("bias".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert("bias".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clipmin(mut self, val: f32) -> Self {
        self.params.insert("clipmin".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_clipmin_expr(mut self, expr: &str) -> Self {
        self.params.insert("clipmin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clipmax(mut self, val: f32) -> Self {
        self.params.insert("clipmax".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_clipmax_expr(mut self, expr: &str) -> Self {
        self.params.insert("clipmax".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_disp(mut self, val: f32) -> Self {
        self.params.insert("disp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_disp_expr(mut self, expr: &str) -> Self {
        self.params.insert("disp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dispfreq(mut self, val: f32) -> Self {
        self.params.insert("dispfreq".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_dispfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert("dispfreq".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gflow(mut self, val: f32) -> Self {
        self.params.insert("gflow".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gflow_expr(mut self, expr: &str) -> Self {
        self.params.insert("gflow".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_elementscale(mut self, val: [f32; 3]) -> Self {
        self.params.insert("elementscale".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_elementscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("elementscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert("offset".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_period(mut self, val: [f32; 3]) -> Self {
        self.params.insert("period".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_period_expr(mut self, expr: &str) -> Self {
        self.params.insert("period".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_combine(mut self, val: SopHeightfieldNoiseCombine) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_combine_expr(mut self, expr: &str) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_layer(mut self, val: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_layer_expr(mut self, expr: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_masklayer(mut self, val: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_masklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_basis(mut self, val: &str) -> Self {
        self.params.insert("basis".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_basis_expr(mut self, expr: &str) -> Self {
        self.params.insert("basis".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fractal(mut self, val: &str) -> Self {
        self.params.insert("fractal".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_fractal_expr(mut self, expr: &str) -> Self {
        self.params.insert("fractal".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_centernoise(mut self, val: bool) -> Self {
        self.params.insert("centernoise".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_centernoise_expr(mut self, expr: &str) -> Self {
        self.params.insert("centernoise".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fold(mut self, val: bool) -> Self {
        self.params.insert("fold".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_fold_expr(mut self, expr: &str) -> Self {
        self.params.insert("fold".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_complement(mut self, val: bool) -> Self {
        self.params.insert("complement".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_complement_expr(mut self, expr: &str) -> Self {
        self.params.insert("complement".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dogain(mut self, val: bool) -> Self {
        self.params.insert("dogain".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dogain_expr(mut self, expr: &str) -> Self {
        self.params.insert("dogain".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dobias(mut self, val: bool) -> Self {
        self.params.insert("dobias".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dobias_expr(mut self, expr: &str) -> Self {
        self.params.insert("dobias".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dolwarp(mut self, val: bool) -> Self {
        self.params.insert("dolwarp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dolwarp_expr(mut self, expr: &str) -> Self {
        self.params.insert("dolwarp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_accuml(mut self, val: bool) -> Self {
        self.params.insert("accuml".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_accuml_expr(mut self, expr: &str) -> Self {
        self.params.insert("accuml".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dogwarp(mut self, val: bool) -> Self {
        self.params.insert("dogwarp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dogwarp_expr(mut self, expr: &str) -> Self {
        self.params.insert("dogwarp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_accumg(mut self, val: bool) -> Self {
        self.params.insert("accumg".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_accumg_expr(mut self, expr: &str) -> Self {
        self.params.insert("accumg".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldNoise {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_noise"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldOutputOutType {
    PackedRaster = 0,
    DeepRaster = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldOutputOutFormat {
    Rgba = 0,
    Rgb = 1,
    SingleChannel = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldOutputOutputType {
    /// 8b Fixed
    N8bFixed = 0,
    /// 16b Fixed
    N16bFixed = 1,
    /// 16b Floating Point
    N16bFloatingPoint = 2,
    /// 32b Floating Point
    N32bFloatingPoint = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldOutputRedOutRange {
    NoRemapping = 0,
    AutoRemap = 1,
    ManualRemap = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldOutputGreenOutRange {
    NoRemapping = 0,
    AutoRemap = 1,
    ManualRemap = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldOutputBlueOutRange {
    NoRemapping = 0,
    AutoRemap = 1,
    ManualRemap = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldOutputAlphaOutRange {
    NoRemapping = 0,
    AutoRemap = 1,
    ManualRemap = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldOutputTileMethod {
    TileSize = 0,
    NumberOfTiles = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldOutputFileNaming {
    Udim = 0,
    UvTile = 1,
    Frames = 2,
    XyTile = 3,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldOutput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldOutput {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert("execute".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float2 parameters ---
    pub fn with_red_from_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert("red_from_range".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_red_from_range_expr(mut self, expr: &str) -> Self {
        self.params.insert("red_from_range".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_red_to_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert("red_to_range".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_red_to_range_expr(mut self, expr: &str) -> Self {
        self.params.insert("red_to_range".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_green_from_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert("green_from_range".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_green_from_range_expr(mut self, expr: &str) -> Self {
        self.params.insert("green_from_range".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_green_to_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert("green_to_range".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_green_to_range_expr(mut self, expr: &str) -> Self {
        self.params.insert("green_to_range".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blue_from_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert("blue_from_range".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_blue_from_range_expr(mut self, expr: &str) -> Self {
        self.params.insert("blue_from_range".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blue_to_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert("blue_to_range".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_blue_to_range_expr(mut self, expr: &str) -> Self {
        self.params.insert("blue_to_range".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_alpha_from_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert("alpha_from_range".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_alpha_from_range_expr(mut self, expr: &str) -> Self {
        self.params.insert("alpha_from_range".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_alpha_to_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert("alpha_to_range".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_alpha_to_range_expr(mut self, expr: &str) -> Self {
        self.params.insert("alpha_to_range".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert("f".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert("f".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_udim_stride(mut self, val: i32) -> Self {
        self.params.insert("udim_stride".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_udim_stride_expr(mut self, expr: &str) -> Self {
        self.params.insert("udim_stride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tile_padding(mut self, val: i32) -> Self {
        self.params.insert("tile_padding".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_tile_padding_expr(mut self, expr: &str) -> Self {
        self.params.insert("tile_padding".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int2 parameters ---
    pub fn with_resolution(mut self, val: [i32; 2]) -> Self {
        self.params.insert("resolution".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_resolution_expr(mut self, expr: &str) -> Self {
        self.params.insert("resolution".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_volume_res(mut self, val: [i32; 2]) -> Self {
        self.params.insert("volume_res".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_volume_res_expr(mut self, expr: &str) -> Self {
        self.params.insert("volume_res".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_actual_res(mut self, val: [i32; 2]) -> Self {
        self.params.insert("actual_res".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_actual_res_expr(mut self, expr: &str) -> Self {
        self.params.insert("actual_res".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_red_auto_range(mut self, val: [i32; 2]) -> Self {
        self.params.insert("red_auto_range".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_red_auto_range_expr(mut self, expr: &str) -> Self {
        self.params.insert("red_auto_range".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_red_final_range(mut self, val: [i32; 2]) -> Self {
        self.params.insert("red_final_range".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_red_final_range_expr(mut self, expr: &str) -> Self {
        self.params.insert("red_final_range".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tile_size(mut self, val: [i32; 2]) -> Self {
        self.params.insert("tile_size".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_tile_size_expr(mut self, expr: &str) -> Self {
        self.params.insert("tile_size".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_num_tiles(mut self, val: [i32; 2]) -> Self {
        self.params.insert("num_tiles".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_num_tiles_expr(mut self, expr: &str) -> Self {
        self.params.insert("num_tiles".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tile_overlap(mut self, val: [i32; 2]) -> Self {
        self.params.insert("tile_overlap".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_tile_overlap_expr(mut self, expr: &str) -> Self {
        self.params.insert("tile_overlap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_actual_tile_size(mut self, val: [i32; 2]) -> Self {
        self.params.insert("actual_tile_size".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_actual_tile_size_expr(mut self, expr: &str) -> Self {
        self.params.insert("actual_tile_size".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_actual_num_tiles(mut self, val: [i32; 2]) -> Self {
        self.params.insert("actual_num_tiles".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_actual_num_tiles_expr(mut self, expr: &str) -> Self {
        self.params.insert("actual_num_tiles".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tile_offset(mut self, val: [i32; 2]) -> Self {
        self.params.insert("tile_offset".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_tile_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("tile_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int3 parameters ---
    pub fn with_out_range(mut self, val: [i32; 3]) -> Self {
        self.params.insert("out_range".to_string(), crate::core::types::ParamValue::Int3(val));
        self
    }
    pub fn with_out_range_expr(mut self, expr: &str) -> Self {
        self.params.insert("out_range".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_out_type(mut self, val: SopHeightfieldOutputOutType) -> Self {
        self.params.insert("out_type".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_out_type_expr(mut self, expr: &str) -> Self {
        self.params.insert("out_type".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_out_format(mut self, val: SopHeightfieldOutputOutFormat) -> Self {
        self.params.insert("out_format".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_out_format_expr(mut self, expr: &str) -> Self {
        self.params.insert("out_format".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_output_type(mut self, val: SopHeightfieldOutputOutputType) -> Self {
        self.params.insert("output_type".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_output_type_expr(mut self, expr: &str) -> Self {
        self.params.insert("output_type".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_red_out_range(mut self, val: SopHeightfieldOutputRedOutRange) -> Self {
        self.params.insert("red_out_range".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_red_out_range_expr(mut self, expr: &str) -> Self {
        self.params.insert("red_out_range".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_green_out_range(mut self, val: SopHeightfieldOutputGreenOutRange) -> Self {
        self.params.insert("green_out_range".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_green_out_range_expr(mut self, expr: &str) -> Self {
        self.params.insert("green_out_range".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blue_out_range(mut self, val: SopHeightfieldOutputBlueOutRange) -> Self {
        self.params.insert("blue_out_range".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_blue_out_range_expr(mut self, expr: &str) -> Self {
        self.params.insert("blue_out_range".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_alpha_out_range(mut self, val: SopHeightfieldOutputAlphaOutRange) -> Self {
        self.params.insert("alpha_out_range".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_alpha_out_range_expr(mut self, expr: &str) -> Self {
        self.params.insert("alpha_out_range".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tile_method(mut self, val: SopHeightfieldOutputTileMethod) -> Self {
        self.params.insert("tile_method".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tile_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("tile_method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_file_naming(mut self, val: SopHeightfieldOutputFileNaming) -> Self {
        self.params.insert("file_naming".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_file_naming_expr(mut self, expr: &str) -> Self {
        self.params.insert("file_naming".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_output(mut self, val: &str) -> Self {
        self.params.insert("output".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_output_expr(mut self, expr: &str) -> Self {
        self.params.insert("output".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_red_channel(mut self, val: &str) -> Self {
        self.params.insert("red_channel".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_red_channel_expr(mut self, expr: &str) -> Self {
        self.params.insert("red_channel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_green_channel(mut self, val: &str) -> Self {
        self.params.insert("green_channel".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_green_channel_expr(mut self, expr: &str) -> Self {
        self.params.insert("green_channel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blue_channel(mut self, val: &str) -> Self {
        self.params.insert("blue_channel".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_blue_channel_expr(mut self, expr: &str) -> Self {
        self.params.insert("blue_channel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_alpha_channel(mut self, val: &str) -> Self {
        self.params.insert("alpha_channel".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_alpha_channel_expr(mut self, expr: &str) -> Self {
        self.params.insert("alpha_channel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_deep_raster_layers(mut self, val: &str) -> Self {
        self.params.insert("deep_raster_layers".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_deep_raster_layers_expr(mut self, expr: &str) -> Self {
        self.params.insert("deep_raster_layers".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_deep_raster_layers01(mut self, val: &str) -> Self {
        self.params.insert("deep_raster_layers01".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_deep_raster_layers01_expr(mut self, expr: &str) -> Self {
        self.params.insert("deep_raster_layers01".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_deep_raster_layers_11(mut self, val: &str) -> Self {
        self.params.insert("deep_raster_layers_11".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_deep_raster_layers_11_expr(mut self, expr: &str) -> Self {
        self.params.insert("deep_raster_layers_11".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_file_out(mut self, val: &str) -> Self {
        self.params.insert("file_out".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_file_out_expr(mut self, expr: &str) -> Self {
        self.params.insert("file_out".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_specify_res(mut self, val: bool) -> Self {
        self.params.insert("specify_res".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_specify_res_expr(mut self, expr: &str) -> Self {
        self.params.insert("specify_res".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_trange(mut self, val: bool) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xflip(mut self, val: bool) -> Self {
        self.params.insert("xflip".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_xflip_expr(mut self, expr: &str) -> Self {
        self.params.insert("xflip".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_yflip(mut self, val: bool) -> Self {
        self.params.insert("yflip".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_yflip_expr(mut self, expr: &str) -> Self {
        self.params.insert("yflip".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flop(mut self, val: bool) -> Self {
        self.params.insert("flop".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_flop_expr(mut self, expr: &str) -> Self {
        self.params.insert("flop".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fill_red_channel(mut self, val: bool) -> Self {
        self.params.insert("fill_red_channel".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_fill_red_channel_expr(mut self, expr: &str) -> Self {
        self.params.insert("fill_red_channel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fill_green_channel(mut self, val: bool) -> Self {
        self.params.insert("fill_green_channel".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_fill_green_channel_expr(mut self, expr: &str) -> Self {
        self.params.insert("fill_green_channel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fill_blue_channel(mut self, val: bool) -> Self {
        self.params.insert("fill_blue_channel".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_fill_blue_channel_expr(mut self, expr: &str) -> Self {
        self.params.insert("fill_blue_channel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fill_alpha_channel(mut self, val: bool) -> Self {
        self.params.insert("fill_alpha_channel".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_fill_alpha_channel_expr(mut self, expr: &str) -> Self {
        self.params.insert("fill_alpha_channel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_logoutput(mut self, val: bool) -> Self {
        self.params.insert("pdg_logoutput".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_pdg_logoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_logoutput".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tile_output(mut self, val: bool) -> Self {
        self.params.insert("tile_output".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tile_output_expr(mut self, expr: &str) -> Self {
        self.params.insert("tile_output".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldOutput {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_output"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("cop2net1")
    }
}
#[allow(clippy::wrong_self_convention)]
pub trait SopHeightfieldOutputInnerExt {
    fn n_16b(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn n_8bit(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rgb(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rgba(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn alpha_volume(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blue_volume(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn chan1_zero(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn chan2_zero(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn chan3_zero(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn chan4_one(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn chan_fit1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn chan_fit2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn chan_fit3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn chan_fit4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn channelcopy1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn channelcopy2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn channelcopy3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn deep_raster_volumes(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn deep_raster_volumes1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn deep_raster_volumes2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn delete1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn delete2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn delete3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn delete4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn delete6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn delete7(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn flip1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn fp16(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn green_volume(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn merge4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn red_volume(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rename1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rename2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rename3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rename4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn single(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn single_out(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch3(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch4(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch5(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch6(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch8(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn tiled_out(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn window1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> SopHeightfieldOutputInnerExt for crate::core::graph::InnerGraph<'a> {
    fn n_16b(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/16b")
    }
    fn n_8bit(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/8bit")
    }
    fn rgb(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/RGB")
    }
    fn rgba(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/RGBA")
    }
    fn alpha_volume(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/alpha_volume")
    }
    fn blue_volume(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/blue_volume")
    }
    fn chan1_zero(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/chan1_zero")
    }
    fn chan2_zero(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/chan2_zero")
    }
    fn chan3_zero(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/chan3_zero")
    }
    fn chan4_one(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/chan4_one")
    }
    fn chan_fit1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/chan_fit1")
    }
    fn chan_fit2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/chan_fit2")
    }
    fn chan_fit3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/chan_fit3")
    }
    fn chan_fit4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/chan_fit4")
    }
    fn channelcopy1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/channelcopy1")
    }
    fn channelcopy2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/channelcopy2")
    }
    fn channelcopy3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/channelcopy3")
    }
    fn deep_raster_volumes(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/deep_raster_volumes")
    }
    fn deep_raster_volumes1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/deep_raster_volumes1")
    }
    fn deep_raster_volumes2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/deep_raster_volumes2")
    }
    fn delete1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/delete1")
    }
    fn delete2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/delete2")
    }
    fn delete3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/delete3")
    }
    fn delete4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/delete4")
    }
    fn delete6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/delete6")
    }
    fn delete7(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/delete7")
    }
    fn flip1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/flip1")
    }
    fn fp16(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/fp16")
    }
    fn green_volume(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/green_volume")
    }
    fn merge1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/merge1")
    }
    fn merge2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/merge2")
    }
    fn merge3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/merge3")
    }
    fn merge4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/merge4")
    }
    fn red_volume(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/red_volume")
    }
    fn rename1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/rename1")
    }
    fn rename2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/rename2")
    }
    fn rename3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/rename3")
    }
    fn rename4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/rename4")
    }
    fn single(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/single")
    }
    fn single_out(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/single_out")
    }
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/switch1")
    }
    fn switch2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/switch2")
    }
    fn switch3(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/switch3")
    }
    fn switch4(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/switch4")
    }
    fn switch5(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/switch5")
    }
    fn switch6(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/switch6")
    }
    fn switch8(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/switch8")
    }
    fn tiled_out(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/tiled_out")
    }
    fn window1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("cop2net1/window1")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldPaintLmboperation {
    PaintFg = 0,
    PaintBg = 1,
    Smooth = 2,
    Sharpen = 3,
    SampleFg = 4,
    SampleBg = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldPaintLmboperation2 {
    PaintFg = 0,
    PaintBg = 1,
    Smooth = 2,
    Sharpen = 3,
    SampleFg = 4,
    SampleBg = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldPaintLmboperation3 {
    PaintFg = 0,
    PaintBg = 1,
    Smooth = 2,
    Sharpen = 3,
    SampleFg = 4,
    SampleBg = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldPaintMmboperation {
    PaintFg = 0,
    PaintBg = 1,
    Smooth = 2,
    Sharpen = 3,
    SampleFg = 4,
    SampleBg = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldPaintMmboperation2 {
    PaintFg = 0,
    PaintBg = 1,
    Smooth = 2,
    Sharpen = 3,
    SampleFg = 4,
    SampleBg = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldPaintMmboperation3 {
    PaintFg = 0,
    PaintBg = 1,
    Smooth = 2,
    Sharpen = 3,
    SampleFg = 4,
    SampleBg = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldPaintPaintmode {
    Replace = 0,
    Add = 1,
    Multiply = 2,
    Maximum = 3,
    Minimum = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldPaintStrokeProjtype {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
    ScreenPlane = 3,
    Geometry = 4,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldPaint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldPaint {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain to Paint"
    pub fn set_input_terrain_to_paint<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain to Paint" and specifies the output index of the target node.
    pub fn set_input_terrain_to_paint_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_reset(mut self) -> Self {
        self.params.insert("reset".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_recache(mut self) -> Self {
        self.params.insert("recache".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_movestashtofile(mut self) -> Self {
        self.params.insert("movestashtofile".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_loadstashfromfile(mut self) -> Self {
        self.params.insert("loadstashfromfile".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Data parameters ---
    pub fn with_strokes(mut self, val: &str) -> Self {
        self.params.insert("strokes".to_string(), crate::core::types::ParamValue::Data(val.to_string()));
        self
    }
    pub fn with_strokes_expr(mut self, expr: &str) -> Self {
        self.params.insert("strokes".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bakedstrokes(mut self, val: &str) -> Self {
        self.params.insert("bakedstrokes".to_string(), crate::core::types::ParamValue::Data(val.to_string()));
        self
    }
    pub fn with_bakedstrokes_expr(mut self, expr: &str) -> Self {
        self.params.insert("bakedstrokes".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unsavedbakedstrokes(mut self, val: &str) -> Self {
        self.params.insert("unsavedbakedstrokes".to_string(), crate::core::types::ParamValue::Data(val.to_string()));
        self
    }
    pub fn with_unsavedbakedstrokes_expr(mut self, expr: &str) -> Self {
        self.params.insert("unsavedbakedstrokes".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float parameters ---
    pub fn with_fgvalue(mut self, val: f32) -> Self {
        self.params.insert("fgvalue".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_fgvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert("fgvalue".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bgvalue(mut self, val: f32) -> Self {
        self.params.insert("bgvalue".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bgvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert("bgvalue".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blurradius(mut self, val: f32) -> Self {
        self.params.insert("blurradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blurradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("blurradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_radius(mut self, val: f32) -> Self {
        self.params.insert("stroke_radius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stroke_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert("stroke_radius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_opacity(mut self, val: f32) -> Self {
        self.params.insert("stroke_opacity".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stroke_opacity_expr(mut self, expr: &str) -> Self {
        self.params.insert("stroke_opacity".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_softedge(mut self, val: f32) -> Self {
        self.params.insert("stroke_softedge".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stroke_softedge_expr(mut self, expr: &str) -> Self {
        self.params.insert("stroke_softedge".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_opacitypressure(mut self, val: f32) -> Self {
        self.params.insert("stroke_opacitypressure".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stroke_opacitypressure_expr(mut self, expr: &str) -> Self {
        self.params.insert("stroke_opacitypressure".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_radiuspressure(mut self, val: f32) -> Self {
        self.params.insert("stroke_radiuspressure".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stroke_radiuspressure_expr(mut self, expr: &str) -> Self {
        self.params.insert("stroke_radiuspressure".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_quality(mut self, val: f32) -> Self {
        self.params.insert("quality".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_quality_expr(mut self, expr: &str) -> Self {
        self.params.insert("quality".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_value(mut self, val: f32) -> Self {
        self.params.insert("stroke_value".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stroke_value_expr(mut self, expr: &str) -> Self {
        self.params.insert("stroke_value".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_radius_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(format!("stroke{}_radius", index1), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stroke_radius_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_radius", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_opacity_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(format!("stroke{}_opacity", index1), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stroke_opacity_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_opacity", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_stroke_color_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(format!("stroke{}_color", index1), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_stroke_color_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_color", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_projcenter_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(format!("stroke{}_projcenter", index1), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_stroke_projcenter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_projcenter", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_projdir_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(format!("stroke{}_projdir", index1), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_stroke_projdir_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_projdir", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_stroke_tool_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(format!("stroke{}_tool", index1), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_stroke_tool_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_tool", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_projtype_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(format!("stroke{}_projtype", index1), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_stroke_projtype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_projtype", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_lmboperation(mut self, val: SopHeightfieldPaintLmboperation) -> Self {
        self.params.insert("lmboperation".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_lmboperation_expr(mut self, expr: &str) -> Self {
        self.params.insert("lmboperation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lmboperation2(mut self, val: SopHeightfieldPaintLmboperation2) -> Self {
        self.params.insert("lmboperation2".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_lmboperation2_expr(mut self, expr: &str) -> Self {
        self.params.insert("lmboperation2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lmboperation3(mut self, val: SopHeightfieldPaintLmboperation3) -> Self {
        self.params.insert("lmboperation3".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_lmboperation3_expr(mut self, expr: &str) -> Self {
        self.params.insert("lmboperation3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mmboperation(mut self, val: SopHeightfieldPaintMmboperation) -> Self {
        self.params.insert("mmboperation".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_mmboperation_expr(mut self, expr: &str) -> Self {
        self.params.insert("mmboperation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mmboperation2(mut self, val: SopHeightfieldPaintMmboperation2) -> Self {
        self.params.insert("mmboperation2".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_mmboperation2_expr(mut self, expr: &str) -> Self {
        self.params.insert("mmboperation2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mmboperation3(mut self, val: SopHeightfieldPaintMmboperation3) -> Self {
        self.params.insert("mmboperation3".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_mmboperation3_expr(mut self, expr: &str) -> Self {
        self.params.insert("mmboperation3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_paintmode(mut self, val: SopHeightfieldPaintPaintmode) -> Self {
        self.params.insert("paintmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_paintmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("paintmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_projtype(mut self, val: SopHeightfieldPaintStrokeProjtype) -> Self {
        self.params.insert("stroke_projtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_stroke_projtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("stroke_projtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_stroke_layer(mut self, val: &str) -> Self {
        self.params.insert("stroke_layer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_stroke_layer_expr(mut self, expr: &str) -> Self {
        self.params.insert("stroke_layer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_method(mut self, val: &str) -> Self {
        self.params.insert("stroke_method".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_stroke_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("stroke_method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_data_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("stroke{}_data", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_stroke_data_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_data", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_metadata_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("stroke{}_metadata", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_stroke_metadata_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_metadata", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_strokesfile(mut self, val: &str) -> Self {
        self.params.insert("strokesfile".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_strokesfile_expr(mut self, expr: &str) -> Self {
        self.params.insert("strokesfile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bakedstrokesfile(mut self, val: &str) -> Self {
        self.params.insert("bakedstrokesfile".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_bakedstrokesfile_expr(mut self, expr: &str) -> Self {
        self.params.insert("bakedstrokesfile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_dotint(mut self, val: bool) -> Self {
        self.params.insert("dotint".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dotint_expr(mut self, expr: &str) -> Self {
        self.params.insert("dotint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_savecache(mut self, val: bool) -> Self {
        self.params.insert("savecache".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_savecache_expr(mut self, expr: &str) -> Self {
        self.params.insert("savecache".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_livemode(mut self, val: bool) -> Self {
        self.params.insert("livemode".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_livemode_expr(mut self, expr: &str) -> Self {
        self.params.insert("livemode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cachedgeometry(mut self, val: bool) -> Self {
        self.params.insert("cachedgeometry".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_cachedgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert("cachedgeometry".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stroke_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("stroke{}_enable", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_stroke_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("stroke{}_enable", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldPaint {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_paint"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone)]
pub struct SopHeightfieldPatch {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldPatch {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Base Terrain"
    pub fn set_input_base_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Base Terrain" and specifies the output index of the target node.
    pub fn set_input_base_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Patching Terrain & Mask"
    pub fn set_input_patching_terrain_mask<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Patching Terrain & Mask" and specifies the output index of the target node.
    pub fn set_input_patching_terrain_mask_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightscale(mut self, val: f32) -> Self {
        self.params.insert("heightscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_heightscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_baseheightscale(mut self, val: f32) -> Self {
        self.params.insert("baseheightscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_baseheightscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("baseheightscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tx(mut self, val: f32) -> Self {
        self.params.insert("tx".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tx_expr(mut self, expr: &str) -> Self {
        self.params.insert("tx".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tz(mut self, val: f32) -> Self {
        self.params.insert("tz".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert("tz".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ry(mut self, val: f32) -> Self {
        self.params.insert("ry".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ry_expr(mut self, expr: &str) -> Self {
        self.params.insert("ry".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_base(mut self, val: &str) -> Self {
        self.params.insert("base".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_base_expr(mut self, expr: &str) -> Self {
        self.params.insert("base".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_patch(mut self, val: &str) -> Self {
        self.params.insert("patch".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_patch_expr(mut self, expr: &str) -> Self {
        self.params.insert("patch".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mask(mut self, val: &str) -> Self {
        self.params.insert("mask".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert("mask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_centerpatch(mut self, val: bool) -> Self {
        self.params.insert("centerpatch".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_centerpatch_expr(mut self, expr: &str) -> Self {
        self.params.insert("centerpatch".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldPatch {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_patch"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldPatternCombine {
    Replace = 0,
    Add = 1,
    Subtract = 2,
    Difference = 3,
    Multiply = 4,
    Maximum = 5,
    Minimum = 6,
    Blend = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldPatternPattern {
    Ramp = 0,
    /// Exp. Ramp
    ExpRamp = 1,
    Steps = 2,
    Stripes = 3,
    Stars = 4,
    Cells = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldPatternRampmode {
    Linear = 0,
    Concentric = 1,
    Radial = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldPatternNoisetype {
    Simplex = 0,
    Curl = 1,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldPattern {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldPattern {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask"
    pub fn set_input_mask<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask" and specifies the output index of the target node.
    pub fn set_input_mask_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_height(mut self, val: f32) -> Self {
        self.params.insert("height".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_height_expr(mut self, expr: &str) -> Self {
        self.params.insert("height".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_baseheight(mut self, val: f32) -> Self {
        self.params.insert("baseheight".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_baseheight_expr(mut self, expr: &str) -> Self {
        self.params.insert("baseheight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_postblurradius(mut self, val: f32) -> Self {
        self.params.insert("postblurradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_postblurradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("postblurradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rotate(mut self, val: f32) -> Self {
        self.params.insert("rotate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert("rotate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_size(mut self, val: f32) -> Self {
        self.params.insert("size".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert("size".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_phase(mut self, val: f32) -> Self {
        self.params.insert("phase".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_phase_expr(mut self, expr: &str) -> Self {
        self.params.insert("phase".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_riseoverrun(mut self, val: f32) -> Self {
        self.params.insert("riseoverrun".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_riseoverrun_expr(mut self, expr: &str) -> Self {
        self.params.insert("riseoverrun".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stepheight(mut self, val: f32) -> Self {
        self.params.insert("stepheight".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stepheight_expr(mut self, expr: &str) -> Self {
        self.params.insert("stepheight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_steprefheight(mut self, val: f32) -> Self {
        self.params.insert("steprefheight".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_steprefheight_expr(mut self, expr: &str) -> Self {
        self.params.insert("steprefheight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stripewidth(mut self, val: f32) -> Self {
        self.params.insert("stripewidth".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stripewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert("stripewidth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shapesize(mut self, val: f32) -> Self {
        self.params.insert("shapesize".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_shapesize_expr(mut self, expr: &str) -> Self {
        self.params.insert("shapesize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shapesizerandom(mut self, val: f32) -> Self {
        self.params.insert("shapesizerandom".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_shapesizerandom_expr(mut self, expr: &str) -> Self {
        self.params.insert("shapesizerandom".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shaperotate(mut self, val: f32) -> Self {
        self.params.insert("shaperotate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_shaperotate_expr(mut self, expr: &str) -> Self {
        self.params.insert("shaperotate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shapeanglerandom(mut self, val: f32) -> Self {
        self.params.insert("shapeanglerandom".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_shapeanglerandom_expr(mut self, expr: &str) -> Self {
        self.params.insert("shapeanglerandom".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shapejitter(mut self, val: f32) -> Self {
        self.params.insert("shapejitter".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_shapejitter_expr(mut self, expr: &str) -> Self {
        self.params.insert("shapejitter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shapesharpness(mut self, val: f32) -> Self {
        self.params.insert("shapesharpness".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_shapesharpness_expr(mut self, expr: &str) -> Self {
        self.params.insert("shapesharpness".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shapehexness(mut self, val: f32) -> Self {
        self.params.insert("shapehexness".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_shapehexness_expr(mut self, expr: &str) -> Self {
        self.params.insert("shapehexness".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_voronoijitter(mut self, val: f32) -> Self {
        self.params.insert("voronoijitter".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_voronoijitter_expr(mut self, expr: &str) -> Self {
        self.params.insert("voronoijitter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_voronoiwall(mut self, val: f32) -> Self {
        self.params.insert("voronoiwall".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_voronoiwall_expr(mut self, expr: &str) -> Self {
        self.params.insert("voronoiwall".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_distort_amp(mut self, val: f32) -> Self {
        self.params.insert("distort_amp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_distort_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert("distort_amp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_distort_element_size(mut self, val: f32) -> Self {
        self.params.insert("distort_element_size".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_distort_element_size_expr(mut self, expr: &str) -> Self {
        self.params.insert("distort_element_size".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_distort_rough(mut self, val: f32) -> Self {
        self.params.insert("distort_rough".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_distort_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert("distort_rough".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_scale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_t(mut self, val: [f32; 2]) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_distort_element_scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert("distort_element_scale".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_distort_element_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("distort_element_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_distort_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert("distort_offset".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_distort_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("distort_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_shapesides(mut self, val: i32) -> Self {
        self.params.insert("shapesides".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_shapesides_expr(mut self, expr: &str) -> Self {
        self.params.insert("shapesides".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_distort_maxoctave(mut self, val: i32) -> Self {
        self.params.insert("distort_maxoctave".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_distort_maxoctave_expr(mut self, expr: &str) -> Self {
        self.params.insert("distort_maxoctave".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_distort_turb(mut self, val: i32) -> Self {
        self.params.insert("distort_turb".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_distort_turb_expr(mut self, expr: &str) -> Self {
        self.params.insert("distort_turb".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_distort_steps(mut self, val: i32) -> Self {
        self.params.insert("distort_steps".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_distort_steps_expr(mut self, expr: &str) -> Self {
        self.params.insert("distort_steps".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_combine(mut self, val: SopHeightfieldPatternCombine) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_combine_expr(mut self, expr: &str) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pattern(mut self, val: SopHeightfieldPatternPattern) -> Self {
        self.params.insert("pattern".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pattern_expr(mut self, expr: &str) -> Self {
        self.params.insert("pattern".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rampmode(mut self, val: SopHeightfieldPatternRampmode) -> Self {
        self.params.insert("rampmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rampmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("rampmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_noisetype(mut self, val: SopHeightfieldPatternNoisetype) -> Self {
        self.params.insert("noisetype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_noisetype_expr(mut self, expr: &str) -> Self {
        self.params.insert("noisetype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_rampremap(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("rampremap".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_rampremap_expr(mut self, expr: &str) -> Self {
        self.params.insert("rampremap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_layer(mut self, val: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_layer_expr(mut self, expr: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_masklayer(mut self, val: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_masklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_ramprepeat(mut self, val: bool) -> Self {
        self.params.insert("ramprepeat".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_ramprepeat_expr(mut self, expr: &str) -> Self {
        self.params.insert("ramprepeat".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rampmirror(mut self, val: bool) -> Self {
        self.params.insert("rampmirror".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_rampmirror_expr(mut self, expr: &str) -> Self {
        self.params.insert("rampmirror".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dodistortion(mut self, val: bool) -> Self {
        self.params.insert("dodistortion".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dodistortion_expr(mut self, expr: &str) -> Self {
        self.params.insert("dodistortion".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldPattern {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_pattern"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldProjectMaskdir {
    EitherSide = 0,
    AboveHeightfield = 1,
    BelowHeightfield = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldProjectCombine {
    Replace = 0,
    Add = 1,
    Multiply = 2,
    Maximum = 3,
    Minimum = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldProjectJittercombine {
    Average = 0,
    Median = 1,
    Shortest = 2,
    Longest = 3,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldProject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldProject {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Geometry to Project"
    pub fn set_input_geometry_to_project<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Geometry to Project" and specifies the output index of the target node.
    pub fn set_input_geometry_to_project_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_maskdensity(mut self, val: f32) -> Self {
        self.params.insert("maskdensity".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maskdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskdensity".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxraydist(mut self, val: f32) -> Self {
        self.params.insert("maxraydist".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maxraydist_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxraydist".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_jitter(mut self, val: f32) -> Self {
        self.params.insert("jitter".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_jitter_expr(mut self, expr: &str) -> Self {
        self.params.insert("jitter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_sample(mut self, val: i32) -> Self {
        self.params.insert("sample".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_sample_expr(mut self, expr: &str) -> Self {
        self.params.insert("sample".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_maskdir(mut self, val: SopHeightfieldProjectMaskdir) -> Self {
        self.params.insert("maskdir".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_maskdir_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskdir".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_combine(mut self, val: SopHeightfieldProjectCombine) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_combine_expr(mut self, expr: &str) -> Self {
        self.params.insert("combine".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_jittercombine(mut self, val: SopHeightfieldProjectJittercombine) -> Self {
        self.params.insert("jittercombine".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_jittercombine_expr(mut self, expr: &str) -> Self {
        self.params.insert("jittercombine".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_layer(mut self, val: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_layer_expr(mut self, expr: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightlayer(mut self, val: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_heightlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_maskmode(mut self, val: bool) -> Self {
        self.params.insert("maskmode".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_maskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert("maskinvert".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskinvert".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hitfarthest(mut self, val: bool) -> Self {
        self.params.insert("hitfarthest".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_hitfarthest_expr(mut self, expr: &str) -> Self {
        self.params.insert("hitfarthest".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dojitter(mut self, val: bool) -> Self {
        self.params.insert("dojitter".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dojitter_expr(mut self, expr: &str) -> Self {
        self.params.insert("dojitter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldProject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_project"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldQuickshadeTinting {
    Unchanged = 0,
    Mask = 1,
    Color = 2,
    None = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldQuickshadeBaseTriplanar {
    TerrainUvCoords = 0,
    Triplanar = 1,
    /// Ortho: X
    OrthoX = 2,
    /// Ortho: Y
    OrthoY = 3,
    /// Ortho: Z
    OrthoZ = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldQuickshadeTex1Triplanar {
    TerrainUvCoords = 0,
    Triplanar = 1,
    /// Ortho: X
    OrthoX = 2,
    /// Ortho: Y
    OrthoY = 3,
    /// Ortho: Z
    OrthoZ = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldQuickshadeTex2Triplanar {
    TerrainUvCoords = 0,
    Triplanar = 1,
    /// Ortho: X
    OrthoX = 2,
    /// Ortho: Y
    OrthoY = 3,
    /// Ortho: Z
    OrthoZ = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldQuickshadeTex3Triplanar {
    TerrainUvCoords = 0,
    Triplanar = 1,
    /// Ortho: X
    OrthoX = 2,
    /// Ortho: Y
    OrthoY = 3,
    /// Ortho: Z
    OrthoZ = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldQuickshadeTex4Triplanar {
    TerrainUvCoords = 0,
    Triplanar = 1,
    /// Ortho: X
    OrthoX = 2,
    /// Ortho: Y
    OrthoY = 3,
    /// Ortho: Z
    OrthoZ = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldQuickshadeTex5Triplanar {
    TerrainUvCoords = 0,
    Triplanar = 1,
    /// Ortho: X
    OrthoX = 2,
    /// Ortho: Y
    OrthoY = 3,
    /// Ortho: Z
    OrthoZ = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldQuickshadeTex6Triplanar {
    TerrainUvCoords = 0,
    Triplanar = 1,
    /// Ortho: X
    OrthoX = 2,
    /// Ortho: Y
    OrthoY = 3,
    /// Ortho: Z
    OrthoZ = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldQuickshadeTex7Triplanar {
    TerrainUvCoords = 0,
    Triplanar = 1,
    /// Ortho: X
    OrthoX = 2,
    /// Ortho: Y
    OrthoY = 3,
    /// Ortho: Z
    OrthoZ = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldQuickshadeTex8Triplanar {
    TerrainUvCoords = 0,
    Triplanar = 1,
    /// Ortho: X
    OrthoX = 2,
    /// Ortho: Y
    OrthoY = 3,
    /// Ortho: Z
    OrthoZ = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldQuickshadeTex9Triplanar {
    TerrainUvCoords = 0,
    Triplanar = 1,
    /// Ortho: X
    OrthoX = 2,
    /// Ortho: Y
    OrthoY = 3,
    /// Ortho: Z
    OrthoZ = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldQuickshadeRoughmapComp {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
    Alpha = 4,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldQuickshade {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldQuickshade {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_ogl_rough(mut self, val: f32) -> Self {
        self.params.insert("ogl_rough".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ogl_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert("ogl_rough".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_base_uv_rot(mut self, val: f32) -> Self {
        self.params.insert("base_uv_rot".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_base_uv_rot_expr(mut self, expr: &str) -> Self {
        self.params.insert("base_uv_rot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_base_tp_scale(mut self, val: f32) -> Self {
        self.params.insert("base_tp_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_base_tp_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("base_tp_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_base_tp_sharp(mut self, val: f32) -> Self {
        self.params.insert("base_tp_sharp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_base_tp_sharp_expr(mut self, expr: &str) -> Self {
        self.params.insert("base_tp_sharp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_base_rough(mut self, val: f32) -> Self {
        self.params.insert("base_rough".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_base_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert("base_rough".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex1_rot(mut self, val: f32) -> Self {
        self.params.insert("tex1_rot".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex1_rot_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex1_rot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex1_tp_scale(mut self, val: f32) -> Self {
        self.params.insert("tex1_tp_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex1_tp_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex1_tp_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex1_tp_sharp(mut self, val: f32) -> Self {
        self.params.insert("tex1_tp_sharp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex1_tp_sharp_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex1_tp_sharp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex1_rough(mut self, val: f32) -> Self {
        self.params.insert("tex1_rough".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex1_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex1_rough".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex2_rot(mut self, val: f32) -> Self {
        self.params.insert("tex2_rot".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex2_rot_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex2_rot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex2_tp_scale(mut self, val: f32) -> Self {
        self.params.insert("tex2_tp_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex2_tp_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex2_tp_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex2_tp_sharp(mut self, val: f32) -> Self {
        self.params.insert("tex2_tp_sharp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex2_tp_sharp_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex2_tp_sharp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex2_rough(mut self, val: f32) -> Self {
        self.params.insert("tex2_rough".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex2_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex2_rough".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex3_rot(mut self, val: f32) -> Self {
        self.params.insert("tex3_rot".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex3_rot_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex3_rot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex3_tp_scale(mut self, val: f32) -> Self {
        self.params.insert("tex3_tp_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex3_tp_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex3_tp_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex3_tp_sharp(mut self, val: f32) -> Self {
        self.params.insert("tex3_tp_sharp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex3_tp_sharp_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex3_tp_sharp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex3_rough(mut self, val: f32) -> Self {
        self.params.insert("tex3_rough".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex3_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex3_rough".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex4_rot(mut self, val: f32) -> Self {
        self.params.insert("tex4_rot".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex4_rot_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex4_rot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex4_tp_scale(mut self, val: f32) -> Self {
        self.params.insert("tex4_tp_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex4_tp_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex4_tp_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex4_tp_sharp(mut self, val: f32) -> Self {
        self.params.insert("tex4_tp_sharp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex4_tp_sharp_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex4_tp_sharp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex4_rough(mut self, val: f32) -> Self {
        self.params.insert("tex4_rough".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex4_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex4_rough".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex5_rot(mut self, val: f32) -> Self {
        self.params.insert("tex5_rot".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex5_rot_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex5_rot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex5_tp_scale(mut self, val: f32) -> Self {
        self.params.insert("tex5_tp_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex5_tp_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex5_tp_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex5_tp_sharp(mut self, val: f32) -> Self {
        self.params.insert("tex5_tp_sharp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex5_tp_sharp_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex5_tp_sharp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex5_rough(mut self, val: f32) -> Self {
        self.params.insert("tex5_rough".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex5_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex5_rough".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex6_rot(mut self, val: f32) -> Self {
        self.params.insert("tex6_rot".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex6_rot_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex6_rot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex6_tp_scale(mut self, val: f32) -> Self {
        self.params.insert("tex6_tp_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex6_tp_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex6_tp_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex6_tp_sharp(mut self, val: f32) -> Self {
        self.params.insert("tex6_tp_sharp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex6_tp_sharp_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex6_tp_sharp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex6_rough(mut self, val: f32) -> Self {
        self.params.insert("tex6_rough".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex6_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex6_rough".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex7_rot(mut self, val: f32) -> Self {
        self.params.insert("tex7_rot".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex7_rot_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex7_rot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex7_tp_scale(mut self, val: f32) -> Self {
        self.params.insert("tex7_tp_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex7_tp_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex7_tp_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex7_tp_sharp(mut self, val: f32) -> Self {
        self.params.insert("tex7_tp_sharp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex7_tp_sharp_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex7_tp_sharp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex7_rough(mut self, val: f32) -> Self {
        self.params.insert("tex7_rough".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex7_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex7_rough".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex8_rot(mut self, val: f32) -> Self {
        self.params.insert("tex8_rot".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex8_rot_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex8_rot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex8_tp_scale(mut self, val: f32) -> Self {
        self.params.insert("tex8_tp_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex8_tp_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex8_tp_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex8_tp_sharp(mut self, val: f32) -> Self {
        self.params.insert("tex8_tp_sharp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex8_tp_sharp_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex8_tp_sharp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex8_rough(mut self, val: f32) -> Self {
        self.params.insert("tex8_rough".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex8_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex8_rough".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex9_rot(mut self, val: f32) -> Self {
        self.params.insert("tex9_rot".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex9_rot_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex9_rot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex9_tp_scale(mut self, val: f32) -> Self {
        self.params.insert("tex9_tp_scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex9_tp_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex9_tp_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex9_tp_sharp(mut self, val: f32) -> Self {
        self.params.insert("tex9_tp_sharp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex9_tp_sharp_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex9_tp_sharp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex9_rough(mut self, val: f32) -> Self {
        self.params.insert("tex9_rough".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tex9_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex9_rough".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_base_uv_scale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("base_uv_scale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_base_uv_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("base_uv_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_base_uv_trans(mut self, val: [f32; 2]) -> Self {
        self.params.insert("base_uv_trans".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_base_uv_trans_expr(mut self, expr: &str) -> Self {
        self.params.insert("base_uv_trans".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex1_scale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex1_scale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex1_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex1_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex1_trans(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex1_trans".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex1_trans_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex1_trans".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex2_scale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex2_scale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex2_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex2_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex2_trans(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex2_trans".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex2_trans_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex2_trans".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex3_scale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex3_scale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex3_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex3_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex3_trans(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex3_trans".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex3_trans_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex3_trans".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex4_scale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex4_scale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex4_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex4_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex4_trans(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex4_trans".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex4_trans_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex4_trans".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex5_scale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex5_scale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex5_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex5_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex5_trans(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex5_trans".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex5_trans_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex5_trans".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex6_scale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex6_scale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex6_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex6_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex6_trans(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex6_trans".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex6_trans_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex6_trans".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex7_scale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex7_scale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex7_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex7_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex7_trans(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex7_trans".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex7_trans_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex7_trans".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex8_scale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex8_scale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex8_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex8_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex8_trans(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex8_trans".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex8_trans_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex8_trans".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex9_scale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex9_scale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex9_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex9_scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex9_trans(mut self, val: [f32; 2]) -> Self {
        self.params.insert("tex9_trans".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_tex9_trans_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex9_trans".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_ogl_amb(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ogl_amb".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ogl_amb_expr(mut self, expr: &str) -> Self {
        self.params.insert("ogl_amb".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ogl_diff(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ogl_diff".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ogl_diff_expr(mut self, expr: &str) -> Self {
        self.params.insert("ogl_diff".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ogl_spec(mut self, val: [f32; 3]) -> Self {
        self.params.insert("ogl_spec".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_ogl_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert("ogl_spec".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_base_tp_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert("base_tp_offset".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_base_tp_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("base_tp_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_base_tp_blend(mut self, val: [f32; 3]) -> Self {
        self.params.insert("base_tp_blend".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_base_tp_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("base_tp_blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex1_tp_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex1_tp_offset".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex1_tp_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex1_tp_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex1_tp_blend(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex1_tp_blend".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex1_tp_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex1_tp_blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex2_tp_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex2_tp_offset".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex2_tp_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex2_tp_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex2_tp_blend(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex2_tp_blend".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex2_tp_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex2_tp_blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex3_tp_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex3_tp_offset".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex3_tp_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex3_tp_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex3_tp_blend(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex3_tp_blend".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex3_tp_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex3_tp_blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex4_tp_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex4_tp_offset".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex4_tp_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex4_tp_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex4_tp_blend(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex4_tp_blend".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex4_tp_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex4_tp_blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex5_tp_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex5_tp_offset".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex5_tp_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex5_tp_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex5_tp_blend(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex5_tp_blend".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex5_tp_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex5_tp_blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex6_tp_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex6_tp_offset".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex6_tp_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex6_tp_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex6_tp_blend(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex6_tp_blend".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex6_tp_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex6_tp_blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex7_tp_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex7_tp_offset".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex7_tp_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex7_tp_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex7_tp_blend(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex7_tp_blend".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex7_tp_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex7_tp_blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex8_tp_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex8_tp_offset".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex8_tp_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex8_tp_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex8_tp_blend(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex8_tp_blend".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex8_tp_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex8_tp_blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex9_tp_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex9_tp_offset".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex9_tp_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex9_tp_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex9_tp_blend(mut self, val: [f32; 3]) -> Self {
        self.params.insert("tex9_tp_blend".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_tex9_tp_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex9_tp_blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float4 parameters ---
    pub fn with_base_tint(mut self, val: [f32; 4]) -> Self {
        self.params.insert("base_tint".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_base_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("base_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex1_tint(mut self, val: [f32; 4]) -> Self {
        self.params.insert("tex1_tint".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_tex1_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex1_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex2_tint(mut self, val: [f32; 4]) -> Self {
        self.params.insert("tex2_tint".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_tex2_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex2_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex3_tint(mut self, val: [f32; 4]) -> Self {
        self.params.insert("tex3_tint".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_tex3_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex3_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex4_tint(mut self, val: [f32; 4]) -> Self {
        self.params.insert("tex4_tint".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_tex4_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex4_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex5_tint(mut self, val: [f32; 4]) -> Self {
        self.params.insert("tex5_tint".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_tex5_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex5_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex6_tint(mut self, val: [f32; 4]) -> Self {
        self.params.insert("tex6_tint".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_tex6_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex6_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex7_tint(mut self, val: [f32; 4]) -> Self {
        self.params.insert("tex7_tint".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_tex7_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex7_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex8_tint(mut self, val: [f32; 4]) -> Self {
        self.params.insert("tex8_tint".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_tex8_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex8_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex9_tint(mut self, val: [f32; 4]) -> Self {
        self.params.insert("tex9_tint".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_tex9_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex9_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_roughmap_comp(mut self, val: SopHeightfieldQuickshadeRoughmapComp) -> Self {
        self.params.insert("roughmap_comp".to_string(), crate::core::types::ParamValue::Int(val as i32));
        self
    }
    pub fn with_roughmap_comp_expr(mut self, expr: &str) -> Self {
        self.params.insert("roughmap_comp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_tinting(mut self, val: SopHeightfieldQuickshadeTinting) -> Self {
        self.params.insert("tinting".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tinting_expr(mut self, expr: &str) -> Self {
        self.params.insert("tinting".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_base_triplanar(mut self, val: SopHeightfieldQuickshadeBaseTriplanar) -> Self {
        self.params.insert("base_triplanar".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_base_triplanar_expr(mut self, expr: &str) -> Self {
        self.params.insert("base_triplanar".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex1_triplanar(mut self, val: SopHeightfieldQuickshadeTex1Triplanar) -> Self {
        self.params.insert("tex1_triplanar".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tex1_triplanar_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex1_triplanar".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex2_triplanar(mut self, val: SopHeightfieldQuickshadeTex2Triplanar) -> Self {
        self.params.insert("tex2_triplanar".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tex2_triplanar_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex2_triplanar".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex3_triplanar(mut self, val: SopHeightfieldQuickshadeTex3Triplanar) -> Self {
        self.params.insert("tex3_triplanar".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tex3_triplanar_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex3_triplanar".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex4_triplanar(mut self, val: SopHeightfieldQuickshadeTex4Triplanar) -> Self {
        self.params.insert("tex4_triplanar".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tex4_triplanar_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex4_triplanar".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex5_triplanar(mut self, val: SopHeightfieldQuickshadeTex5Triplanar) -> Self {
        self.params.insert("tex5_triplanar".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tex5_triplanar_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex5_triplanar".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex6_triplanar(mut self, val: SopHeightfieldQuickshadeTex6Triplanar) -> Self {
        self.params.insert("tex6_triplanar".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tex6_triplanar_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex6_triplanar".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex7_triplanar(mut self, val: SopHeightfieldQuickshadeTex7Triplanar) -> Self {
        self.params.insert("tex7_triplanar".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tex7_triplanar_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex7_triplanar".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex8_triplanar(mut self, val: SopHeightfieldQuickshadeTex8Triplanar) -> Self {
        self.params.insert("tex8_triplanar".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tex8_triplanar_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex8_triplanar".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex9_triplanar(mut self, val: SopHeightfieldQuickshadeTex9Triplanar) -> Self {
        self.params.insert("tex9_triplanar".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_tex9_triplanar_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex9_triplanar".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_ogl_spec_model(mut self, val: &str) -> Self {
        self.params.insert("ogl_spec_model".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_ogl_spec_model_expr(mut self, expr: &str) -> Self {
        self.params.insert("ogl_spec_model".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_base_map(mut self, val: &str) -> Self {
        self.params.insert("base_map".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_base_map_expr(mut self, expr: &str) -> Self {
        self.params.insert("base_map".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex1_mask(mut self, val: &str) -> Self {
        self.params.insert("tex1_mask".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex1_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex1_mask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex1_map(mut self, val: &str) -> Self {
        self.params.insert("tex1_map".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex1_map_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex1_map".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex2_mask(mut self, val: &str) -> Self {
        self.params.insert("tex2_mask".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex2_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex2_mask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex2_map(mut self, val: &str) -> Self {
        self.params.insert("tex2_map".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex2_map_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex2_map".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex3_mask(mut self, val: &str) -> Self {
        self.params.insert("tex3_mask".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex3_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex3_mask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex3_map(mut self, val: &str) -> Self {
        self.params.insert("tex3_map".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex3_map_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex3_map".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex4_mask(mut self, val: &str) -> Self {
        self.params.insert("tex4_mask".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex4_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex4_mask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex4_map(mut self, val: &str) -> Self {
        self.params.insert("tex4_map".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex4_map_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex4_map".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex5_mask(mut self, val: &str) -> Self {
        self.params.insert("tex5_mask".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex5_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex5_mask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex5_map(mut self, val: &str) -> Self {
        self.params.insert("tex5_map".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex5_map_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex5_map".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex6_mask(mut self, val: &str) -> Self {
        self.params.insert("tex6_mask".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex6_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex6_mask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex6_map(mut self, val: &str) -> Self {
        self.params.insert("tex6_map".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex6_map_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex6_map".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex7_mask(mut self, val: &str) -> Self {
        self.params.insert("tex7_mask".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex7_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex7_mask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex7_map(mut self, val: &str) -> Self {
        self.params.insert("tex7_map".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex7_map_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex7_map".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex8_mask(mut self, val: &str) -> Self {
        self.params.insert("tex8_mask".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex8_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex8_mask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex8_map(mut self, val: &str) -> Self {
        self.params.insert("tex8_map".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex8_map_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex8_map".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex9_mask(mut self, val: &str) -> Self {
        self.params.insert("tex9_mask".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex9_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex9_mask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex9_map(mut self, val: &str) -> Self {
        self.params.insert("tex9_map".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tex9_map_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex9_map".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_roughmap(mut self, val: &str) -> Self {
        self.params.insert("roughmap".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_roughmap_expr(mut self, expr: &str) -> Self {
        self.params.insert("roughmap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_specmap(mut self, val: &str) -> Self {
        self.params.insert("specmap".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_specmap_expr(mut self, expr: &str) -> Self {
        self.params.insert("specmap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_emissionmap(mut self, val: &str) -> Self {
        self.params.insert("emissionmap".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_emissionmap_expr(mut self, expr: &str) -> Self {
        self.params.insert("emissionmap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_use_base_map(mut self, val: bool) -> Self {
        self.params.insert("use_base_map".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_base_map_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_base_map".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_base_clamp_uvs(mut self, val: bool) -> Self {
        self.params.insert("base_clamp_uvs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_base_clamp_uvs_expr(mut self, expr: &str) -> Self {
        self.params.insert("base_clamp_uvs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_base_use_tint(mut self, val: bool) -> Self {
        self.params.insert("base_use_tint".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_base_use_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("base_use_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_tex1(mut self, val: bool) -> Self {
        self.params.insert("use_tex1".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_tex1_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_tex1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex_clamp_uvs1(mut self, val: bool) -> Self {
        self.params.insert("tex_clamp_uvs1".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex_clamp_uvs1_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex_clamp_uvs1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex1_use_tint(mut self, val: bool) -> Self {
        self.params.insert("tex1_use_tint".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex1_use_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex1_use_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_tex2(mut self, val: bool) -> Self {
        self.params.insert("use_tex2".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_tex2_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_tex2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex_clamp_uvs2(mut self, val: bool) -> Self {
        self.params.insert("tex_clamp_uvs2".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex_clamp_uvs2_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex_clamp_uvs2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex2_use_tint(mut self, val: bool) -> Self {
        self.params.insert("tex2_use_tint".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex2_use_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex2_use_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_tex3(mut self, val: bool) -> Self {
        self.params.insert("use_tex3".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_tex3_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_tex3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex_clamp_uvs3(mut self, val: bool) -> Self {
        self.params.insert("tex_clamp_uvs3".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex_clamp_uvs3_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex_clamp_uvs3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex3_use_tint(mut self, val: bool) -> Self {
        self.params.insert("tex3_use_tint".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex3_use_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex3_use_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_tex4(mut self, val: bool) -> Self {
        self.params.insert("use_tex4".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_tex4_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_tex4".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex_clamp_uvs4(mut self, val: bool) -> Self {
        self.params.insert("tex_clamp_uvs4".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex_clamp_uvs4_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex_clamp_uvs4".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex4_use_tint(mut self, val: bool) -> Self {
        self.params.insert("tex4_use_tint".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex4_use_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex4_use_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_tex5(mut self, val: bool) -> Self {
        self.params.insert("use_tex5".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_tex5_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_tex5".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex_clamp_uvs5(mut self, val: bool) -> Self {
        self.params.insert("tex_clamp_uvs5".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex_clamp_uvs5_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex_clamp_uvs5".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex5_use_tint(mut self, val: bool) -> Self {
        self.params.insert("tex5_use_tint".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex5_use_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex5_use_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_tex6(mut self, val: bool) -> Self {
        self.params.insert("use_tex6".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_tex6_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_tex6".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex_clamp_uvs6(mut self, val: bool) -> Self {
        self.params.insert("tex_clamp_uvs6".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex_clamp_uvs6_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex_clamp_uvs6".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex6_use_tint(mut self, val: bool) -> Self {
        self.params.insert("tex6_use_tint".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex6_use_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex6_use_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_tex7(mut self, val: bool) -> Self {
        self.params.insert("use_tex7".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_tex7_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_tex7".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex_clamp_uvs7(mut self, val: bool) -> Self {
        self.params.insert("tex_clamp_uvs7".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex_clamp_uvs7_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex_clamp_uvs7".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex7_use_tint(mut self, val: bool) -> Self {
        self.params.insert("tex7_use_tint".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex7_use_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex7_use_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_tex8(mut self, val: bool) -> Self {
        self.params.insert("use_tex8".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_tex8_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_tex8".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex_clamp_uvs8(mut self, val: bool) -> Self {
        self.params.insert("tex_clamp_uvs8".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex_clamp_uvs8_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex_clamp_uvs8".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex8_use_tint(mut self, val: bool) -> Self {
        self.params.insert("tex8_use_tint".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex8_use_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex8_use_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_tex9(mut self, val: bool) -> Self {
        self.params.insert("use_tex9".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_tex9_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_tex9".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex_clamp_uvs9(mut self, val: bool) -> Self {
        self.params.insert("tex_clamp_uvs9".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex_clamp_uvs9_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex_clamp_uvs9".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tex9_use_tint(mut self, val: bool) -> Self {
        self.params.insert("tex9_use_tint".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tex9_use_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert("tex9_use_tint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_roughmap(mut self, val: bool) -> Self {
        self.params.insert("use_roughmap".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_roughmap_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_roughmap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_invert_roughmap(mut self, val: bool) -> Self {
        self.params.insert("invert_roughmap".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_invert_roughmap_expr(mut self, expr: &str) -> Self {
        self.params.insert("invert_roughmap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_specmap(mut self, val: bool) -> Self {
        self.params.insert("use_specmap".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_specmap_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_specmap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ogl_useemissionmap(mut self, val: bool) -> Self {
        self.params.insert("ogl_useemissionmap".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_ogl_useemissionmap_expr(mut self, expr: &str) -> Self {
        self.params.insert("ogl_useemissionmap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldQuickshade {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_quickshade"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone)]
pub struct SopHeightfieldRemap {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldRemap {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain to Remap"
    pub fn set_input_terrain_to_remap<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain to Remap" and specifies the output index of the target node.
    pub fn set_input_terrain_to_remap_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask Terrain"
    pub fn set_input_mask_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask Terrain" and specifies the output index of the target node.
    pub fn set_input_mask_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_computerange(mut self) -> Self {
        self.params.insert("computerange".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_inputmin(mut self, val: f32) -> Self {
        self.params.insert("inputmin".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_inputmin_expr(mut self, expr: &str) -> Self {
        self.params.insert("inputmin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_inputmax(mut self, val: f32) -> Self {
        self.params.insert("inputmax".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_inputmax_expr(mut self, expr: &str) -> Self {
        self.params.insert("inputmax".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputmin(mut self, val: f32) -> Self {
        self.params.insert("outputmin".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_outputmin_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputmin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputmax(mut self, val: f32) -> Self {
        self.params.insert("outputmax".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_outputmax_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputmax".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_remap(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("remap".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_remap_expr(mut self, expr: &str) -> Self {
        self.params.insert("remap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_layer(mut self, val: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_layer_expr(mut self, expr: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_masklayer(mut self, val: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_masklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_clampmin(mut self, val: bool) -> Self {
        self.params.insert("clampmin".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_clampmin_expr(mut self, expr: &str) -> Self {
        self.params.insert("clampmin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clampmax(mut self, val: bool) -> Self {
        self.params.insert("clampmax".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_clampmax_expr(mut self, expr: &str) -> Self {
        self.params.insert("clampmax".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldRemap {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_remap"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldResampleDivisionmode {
    ByAxis = 0,
    BySize = 1,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldResample {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldResample {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_resscale(mut self, val: f32) -> Self {
        self.params.insert("resscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_resscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("resscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridspacing(mut self, val: f32) -> Self {
        self.params.insert("gridspacing".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gridspacing_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridspacing".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_filterscale(mut self, val: f32) -> Self {
        self.params.insert("filterscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_filterscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("filterscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_gridsamples(mut self, val: i32) -> Self {
        self.params.insert("gridsamples".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_gridsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridsamples".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_divisionmode(mut self, val: SopHeightfieldResampleDivisionmode) -> Self {
        self.params.insert("divisionmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_divisionmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("divisionmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_filter(mut self, val: &str) -> Self {
        self.params.insert("filter".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert("filter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_fixedresample(mut self, val: bool) -> Self {
        self.params.insert("fixedresample".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_fixedresample_expr(mut self, expr: &str) -> Self {
        self.params.insert("fixedresample".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldResample {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_resample"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldScatterScattermethod {
    ByCoverageUsingMaskLayer = 0,
    ByDensityUsingMaskLayer = 1,
    TotalPointCountUsingMaskLayer = 2,
    PerPointCountUsingSourcePoints = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldScatterPerpointcountMethod {
    PoissonDistribution = 0,
    ExactNumber = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldScatterPositioningMethod {
    Offset = 0,
    Origin = 1,
    Ratio = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldScatterVariabilityMethod {
    UniformDistribution = 0,
    NormalDistribution = 1,
    ExactScale = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldScatterRelaxPointremovalmethod {
    OnlyFlag = 0,
    Remove = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldScatterPiecemode {
    FromAttribute = 0,
    FromConnectivity = 1,
    SinglePiece = 2,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldScatter {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldScatter {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask or Scatter Points"
    pub fn set_input_mask_or_scatter_points<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask or Scatter Points" and specifies the output index of the target node.
    pub fn set_input_mask_or_scatter_points_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Primitives to Instance"
    pub fn set_input_primitives_to_instance<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Primitives to Instance" and specifies the output index of the target node.
    pub fn set_input_primitives_to_instance_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_coverage(mut self, val: f32) -> Self {
        self.params.insert("coverage".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_coverage_expr(mut self, expr: &str) -> Self {
        self.params.insert("coverage".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert("density".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert("density".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_perpointcount_exactnumber(mut self, val: f32) -> Self {
        self.params.insert("perpointcount_exactnumber".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_perpointcount_exactnumber_expr(mut self, expr: &str) -> Self {
        self.params.insert("perpointcount_exactnumber".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outerradius(mut self, val: f32) -> Self {
        self.params.insert("outerradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_outerradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("outerradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_falloff(mut self, val: f32) -> Self {
        self.params.insert("falloff".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_falloff_expr(mut self, expr: &str) -> Self {
        self.params.insert("falloff".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_variability_exactscale(mut self, val: f32) -> Self {
        self.params.insert("variability_exactscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_variability_exactscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("variability_exactscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_variability_normalspread(mut self, val: f32) -> Self {
        self.params.insert("variability_normalspread".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_variability_normalspread_expr(mut self, expr: &str) -> Self {
        self.params.insert("variability_normalspread".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_relax_maskcutoff(mut self, val: f32) -> Self {
        self.params.insert("relax_maskcutoff".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_relax_maskcutoff_expr(mut self, expr: &str) -> Self {
        self.params.insert("relax_maskcutoff".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_relax_removingrate(mut self, val: f32) -> Self {
        self.params.insert("relax_removingrate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_relax_removingrate_expr(mut self, expr: &str) -> Self {
        self.params.insert("relax_removingrate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_relax_stepratio(mut self, val: f32) -> Self {
        self.params.insert("relax_stepratio".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_relax_stepratio_expr(mut self, expr: &str) -> Self {
        self.params.insert("relax_stepratio".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_randomup(mut self, val: f32) -> Self {
        self.params.insert("randomup".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_randomup_expr(mut self, expr: &str) -> Self {
        self.params.insert("randomup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_randomyaw(mut self, val: f32) -> Self {
        self.params.insert("randomyaw".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_randomyaw_expr(mut self, expr: &str) -> Self {
        self.params.insert("randomyaw".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_quant(mut self, val: f32) -> Self {
        self.params.insert("quant".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_quant_expr(mut self, expr: &str) -> Self {
        self.params.insert("quant".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_positioning_origin(mut self, val: [f32; 2]) -> Self {
        self.params.insert("positioning_origin".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_positioning_origin_expr(mut self, expr: &str) -> Self {
        self.params.insert("positioning_origin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_positioning_offset(mut self, val: [f32; 2]) -> Self {
        self.params.insert("positioning_offset".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_positioning_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("positioning_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_positioning_ratio(mut self, val: [f32; 2]) -> Self {
        self.params.insert("positioning_ratio".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_positioning_ratio_expr(mut self, expr: &str) -> Self {
        self.params.insert("positioning_ratio".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_variability_unifromrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("variability_unifromrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_variability_unifromrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("variability_unifromrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_variability_normalrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("variability_normalrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_variability_normalrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("variability_normalrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_totalpointcount(mut self, val: i32) -> Self {
        self.params.insert("totalpointcount".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_totalpointcount_expr(mut self, expr: &str) -> Self {
        self.params.insert("totalpointcount".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_relax_iterations(mut self, val: i32) -> Self {
        self.params.insert("relax_iterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_relax_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("relax_iterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_seed(mut self, val: i32) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_emergencylimit(mut self, val: i32) -> Self {
        self.params.insert("emergencylimit".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_emergencylimit_expr(mut self, expr: &str) -> Self {
        self.params.insert("emergencylimit".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int2 parameters ---
    pub fn with_perpointcount_poissonrange(mut self, val: [i32; 2]) -> Self {
        self.params.insert("perpointcount_poissonrange".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_perpointcount_poissonrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("perpointcount_poissonrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_scattermethod(mut self, val: SopHeightfieldScatterScattermethod) -> Self {
        self.params.insert("scattermethod".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_scattermethod_expr(mut self, expr: &str) -> Self {
        self.params.insert("scattermethod".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_perpointcount_method(mut self, val: SopHeightfieldScatterPerpointcountMethod) -> Self {
        self.params.insert("perpointcount_method".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_perpointcount_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("perpointcount_method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_positioning_method(mut self, val: SopHeightfieldScatterPositioningMethod) -> Self {
        self.params.insert("positioning_method".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_positioning_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("positioning_method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_variability_method(mut self, val: SopHeightfieldScatterVariabilityMethod) -> Self {
        self.params.insert("variability_method".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_variability_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("variability_method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_relax_pointremovalmethod(mut self, val: SopHeightfieldScatterRelaxPointremovalmethod) -> Self {
        self.params.insert("relax_pointremovalmethod".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_relax_pointremovalmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert("relax_pointremovalmethod".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_piecemode(mut self, val: SopHeightfieldScatterPiecemode) -> Self {
        self.params.insert("piecemode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_piecemode_expr(mut self, expr: &str) -> Self {
        self.params.insert("piecemode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_tag(mut self, val: &str) -> Self {
        self.params.insert("tag".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tag_expr(mut self, expr: &str) -> Self {
        self.params.insert("tag".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_layer(mut self, val: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_layer_expr(mut self, expr: &str) -> Self {
        self.params.insert("layer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sourcetag(mut self, val: &str) -> Self {
        self.params.insert("sourcetag".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sourcetag_expr(mut self, expr: &str) -> Self {
        self.params.insert("sourcetag".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_relax_avoidtag(mut self, val: &str) -> Self {
        self.params.insert("relax_avoidtag".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_relax_avoidtag_expr(mut self, expr: &str) -> Self {
        self.params.insert("relax_avoidtag".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pieceattrib(mut self, val: &str) -> Self {
        self.params.insert("pieceattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pieceattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("pieceattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_relax_points(mut self, val: bool) -> Self {
        self.params.insert("relax_points".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_relax_points_expr(mut self, expr: &str) -> Self {
        self.params.insert("relax_points".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_relax_selfoverlap(mut self, val: bool) -> Self {
        self.params.insert("relax_selfoverlap".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_relax_selfoverlap_expr(mut self, expr: &str) -> Self {
        self.params.insert("relax_selfoverlap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_relax_allowoutofbounds(mut self, val: bool) -> Self {
        self.params.insert("relax_allowoutofbounds".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_relax_allowoutofbounds_expr(mut self, expr: &str) -> Self {
        self.params.insert("relax_allowoutofbounds".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_keepscatterpoints(mut self, val: bool) -> Self {
        self.params.insert("keepscatterpoints".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keepscatterpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert("keepscatterpoints".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_keepterrain(mut self, val: bool) -> Self {
        self.params.insert("keepterrain".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keepterrain_expr(mut self, expr: &str) -> Self {
        self.params.insert("keepterrain".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_matchnormalterrain(mut self, val: bool) -> Self {
        self.params.insert("matchnormalterrain".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_matchnormalterrain_expr(mut self, expr: &str) -> Self {
        self.params.insert("matchnormalterrain".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_matchslopeterrain(mut self, val: bool) -> Self {
        self.params.insert("matchslopeterrain".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_matchslopeterrain_expr(mut self, expr: &str) -> Self {
        self.params.insert("matchslopeterrain".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_instancenewpoints(mut self, val: bool) -> Self {
        self.params.insert("instancenewpoints".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_instancenewpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert("instancenewpoints".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useemergencylimit(mut self, val: bool) -> Self {
        self.params.insert("useemergencylimit".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useemergencylimit_expr(mut self, expr: &str) -> Self {
        self.params.insert("useemergencylimit".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldScatter {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_scatter"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldSlumpSlumpmode {
    Smooth = 0,
    Granular = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldSlumpReposeanglemaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldSlumpGridbiasmaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldSlumpErodabilitymaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldSlumpBankanglemaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldSlumpRemovalratemaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldSlumpBedDepositionratemaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldSlump {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldSlump {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Bedrock"
    pub fn set_input_bedrock<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Bedrock" and specifies the output index of the target node.
    pub fn set_input_bedrock_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_spread_rate(mut self, val: f32) -> Self {
        self.params.insert("spread_rate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_spread_rate_expr(mut self, expr: &str) -> Self {
        self.params.insert("spread_rate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_repose_angle(mut self, val: f32) -> Self {
        self.params.insert("repose_angle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_repose_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert("repose_angle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_quant(mut self, val: f32) -> Self {
        self.params.insert("quant".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_quant_expr(mut self, expr: &str) -> Self {
        self.params.insert("quant".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridbias(mut self, val: f32) -> Self {
        self.params.insert("gridbias".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gridbias_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridbias".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_globalerosionrate(mut self, val: f32) -> Self {
        self.params.insert("globalerosionrate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_globalerosionrate_expr(mut self, expr: &str) -> Self {
        self.params.insert("globalerosionrate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erodability(mut self, val: f32) -> Self {
        self.params.insert("erodability".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_erodability_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodability".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erosionrate(mut self, val: f32) -> Self {
        self.params.insert("erosionrate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_erosionrate_expr(mut self, expr: &str) -> Self {
        self.params.insert("erosionrate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bankangle(mut self, val: f32) -> Self {
        self.params.insert("bankangle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bankangle_expr(mut self, expr: &str) -> Self {
        self.params.insert("bankangle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_height_factor(mut self, val: f32) -> Self {
        self.params.insert("height_factor".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_height_factor_expr(mut self, expr: &str) -> Self {
        self.params.insert("height_factor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_entrainment(mut self, val: f32) -> Self {
        self.params.insert("entrainment".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_entrainment_expr(mut self, expr: &str) -> Self {
        self.params.insert("entrainment".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_removalrate(mut self, val: f32) -> Self {
        self.params.insert("removalrate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_removalrate_expr(mut self, expr: &str) -> Self {
        self.params.insert("removalrate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxdebrisdepth(mut self, val: f32) -> Self {
        self.params.insert("maxdebrisdepth".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maxdebrisdepth_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxdebrisdepth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erodability_initialfactor(mut self, val: f32) -> Self {
        self.params.insert("erodability_initialfactor".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_erodability_initialfactor_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodability_initialfactor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erodability_slopefactor(mut self, val: f32) -> Self {
        self.params.insert("erodability_slopefactor".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_erodability_slopefactor_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodability_slopefactor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bed_erosionratefactor(mut self, val: f32) -> Self {
        self.params.insert("bed_erosionratefactor".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bed_erosionratefactor_expr(mut self, expr: &str) -> Self {
        self.params.insert("bed_erosionratefactor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bed_depositionrate(mut self, val: f32) -> Self {
        self.params.insert("bed_depositionrate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bed_depositionrate_expr(mut self, expr: &str) -> Self {
        self.params.insert("bed_depositionrate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bed_sedimentcap(mut self, val: f32) -> Self {
        self.params.insert("bed_sedimentcap".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bed_sedimentcap_expr(mut self, expr: &str) -> Self {
        self.params.insert("bed_sedimentcap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank_erosionratefactor(mut self, val: f32) -> Self {
        self.params.insert("bank_erosionratefactor".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_erosionratefactor_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank_erosionratefactor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank_maxbankbedwaterratio(mut self, val: f32) -> Self {
        self.params.insert("bank_maxbankbedwaterratio".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_maxbankbedwaterratio_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank_maxbankbedwaterratio".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flow_smooth_rate(mut self, val: f32) -> Self {
        self.params.insert("flow_smooth_rate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_flow_smooth_rate_expr(mut self, expr: &str) -> Self {
        self.params.insert("flow_smooth_rate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erodability_rampupiters(mut self, val: i32) -> Self {
        self.params.insert("erodability_rampupiters".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_erodability_rampupiters_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodability_rampupiters".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flow_smooth_iters(mut self, val: i32) -> Self {
        self.params.insert("flow_smooth_iters".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_flow_smooth_iters_expr(mut self, expr: &str) -> Self {
        self.params.insert("flow_smooth_iters".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_slumpmode(mut self, val: SopHeightfieldSlumpSlumpmode) -> Self {
        self.params.insert("slumpmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_slumpmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("slumpmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_reposeanglemaskmode(mut self, val: SopHeightfieldSlumpReposeanglemaskmode) -> Self {
        self.params.insert("reposeanglemaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_reposeanglemaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("reposeanglemaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridbiasmaskmode(mut self, val: SopHeightfieldSlumpGridbiasmaskmode) -> Self {
        self.params.insert("gridbiasmaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_gridbiasmaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridbiasmaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erodabilitymaskmode(mut self, val: SopHeightfieldSlumpErodabilitymaskmode) -> Self {
        self.params.insert("erodabilitymaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_erodabilitymaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodabilitymaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bankanglemaskmode(mut self, val: SopHeightfieldSlumpBankanglemaskmode) -> Self {
        self.params.insert("bankanglemaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_bankanglemaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("bankanglemaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_removalratemaskmode(mut self, val: SopHeightfieldSlumpRemovalratemaskmode) -> Self {
        self.params.insert("removalratemaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_removalratemaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("removalratemaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bed_depositionratemaskmode(mut self, val: SopHeightfieldSlumpBedDepositionratemaskmode) -> Self {
        self.params.insert("bed_depositionratemaskmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_bed_depositionratemaskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("bed_depositionratemaskmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_reposeanglemasklayer(mut self, val: &str) -> Self {
        self.params.insert("reposeanglemasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_reposeanglemasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("reposeanglemasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stabilitymasklayer(mut self, val: &str) -> Self {
        self.params.insert("stabilitymasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_stabilitymasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("stabilitymasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridbiasmasklayer(mut self, val: &str) -> Self {
        self.params.insert("gridbiasmasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_gridbiasmasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridbiasmasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erodabilitymasklayer(mut self, val: &str) -> Self {
        self.params.insert("erodabilitymasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_erodabilitymasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("erodabilitymasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bankanglemasklayer(mut self, val: &str) -> Self {
        self.params.insert("bankanglemasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_bankanglemasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("bankanglemasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_removalratemasklayer(mut self, val: &str) -> Self {
        self.params.insert("removalratemasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_removalratemasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("removalratemasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bed_depositionratemasklayer(mut self, val: &str) -> Self {
        self.params.insert("bed_depositionratemasklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_bed_depositionratemasklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("bed_depositionratemasklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bedrocklayer(mut self, val: &str) -> Self {
        self.params.insert("bedrocklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_bedrocklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("bedrocklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_materiallayer(mut self, val: &str) -> Self {
        self.params.insert("materiallayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_materiallayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("materiallayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_entrainedlayer(mut self, val: &str) -> Self {
        self.params.insert("entrainedlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_entrainedlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("entrainedlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sedimentlayer(mut self, val: &str) -> Self {
        self.params.insert("sedimentlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sedimentlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("sedimentlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flowlayer(mut self, val: &str) -> Self {
        self.params.insert("flowlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_flowlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("flowlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flowdirlayer(mut self, val: &str) -> Self {
        self.params.insert("flowdirlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_flowdirlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("flowdirlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_doerosion(mut self, val: bool) -> Self {
        self.params.insert("doerosion".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_doerosion_expr(mut self, expr: &str) -> Self {
        self.params.insert("doerosion".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_openborder(mut self, val: bool) -> Self {
        self.params.insert("openborder".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_openborder_expr(mut self, expr: &str) -> Self {
        self.params.insert("openborder".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_add_to_bedrock(mut self, val: bool) -> Self {
        self.params.insert("add_to_bedrock".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_add_to_bedrock_expr(mut self, expr: &str) -> Self {
        self.params.insert("add_to_bedrock".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flow_enable(mut self, val: bool) -> Self {
        self.params.insert("flow_enable".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_flow_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert("flow_enable".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldSlump {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_slump"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldTerraceUndulationType {
    None = 0,
    Standard = 1,
    Custom = 2,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldTerrace {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldTerrace {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask"
    pub fn set_input_mask<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask" and specifies the output index of the target node.
    pub fn set_input_mask_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_computerange(mut self) -> Self {
        self.params.insert("computerange".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_minheight(mut self, val: f32) -> Self {
        self.params.insert("minheight".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_minheight_expr(mut self, expr: &str) -> Self {
        self.params.insert("minheight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxheight(mut self, val: f32) -> Self {
        self.params.insert("maxheight".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maxheight_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxheight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_terrace_fade(mut self, val: f32) -> Self {
        self.params.insert("terrace_fade".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_terrace_fade_expr(mut self, expr: &str) -> Self {
        self.params.insert("terrace_fade".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_terrace_max_step_size(mut self, val: f32) -> Self {
        self.params.insert("terrace_max_step_size".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_terrace_max_step_size_expr(mut self, expr: &str) -> Self {
        self.params.insert("terrace_max_step_size".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_terraceoffset(mut self, val: f32) -> Self {
        self.params.insert("terraceoffset".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_terraceoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert("terraceoffset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_smoothedges(mut self, val: f32) -> Self {
        self.params.insert("smoothedges".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_smoothedges_expr(mut self, expr: &str) -> Self {
        self.params.insert("smoothedges".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_min_mask(mut self, val: f32) -> Self {
        self.params.insert("min_mask".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_min_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert("min_mask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_amp(mut self, val: f32) -> Self {
        self.params.insert("amp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert("amp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_elementsize(mut self, val: f32) -> Self {
        self.params.insert("elementsize".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_elementsize_expr(mut self, expr: &str) -> Self {
        self.params.insert("elementsize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_slopesmoothradius(mut self, val: f32) -> Self {
        self.params.insert("slopesmoothradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_slopesmoothradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("slopesmoothradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mesa_maxslope(mut self, val: f32) -> Self {
        self.params.insert("mesa_maxslope".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_mesa_maxslope_expr(mut self, expr: &str) -> Self {
        self.params.insert("mesa_maxslope".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cliff_minslope(mut self, val: f32) -> Self {
        self.params.insert("cliff_minslope".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_cliff_minslope_expr(mut self, expr: &str) -> Self {
        self.params.insert("cliff_minslope".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert("offset".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_undulation_type(mut self, val: SopHeightfieldTerraceUndulationType) -> Self {
        self.params.insert("undulation_type".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_undulation_type_expr(mut self, expr: &str) -> Self {
        self.params.insert("undulation_type".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_fade_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("fade_ramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_fade_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("fade_ramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_terrace_step(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("terrace_step".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_terrace_step_expr(mut self, expr: &str) -> Self {
        self.params.insert("terrace_step".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_custom_noise_step(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("custom_noise_step".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_custom_noise_step_expr(mut self, expr: &str) -> Self {
        self.params.insert("custom_noise_step".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_heightlayer(mut self, val: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_heightlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_masklayer(mut self, val: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_masklayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("masklayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_basis(mut self, val: &str) -> Self {
        self.params.insert("basis".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_basis_expr(mut self, expr: &str) -> Self {
        self.params.insert("basis".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fractal(mut self, val: &str) -> Self {
        self.params.insert("fractal".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_fractal_expr(mut self, expr: &str) -> Self {
        self.params.insert("fractal".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mesalayer(mut self, val: &str) -> Self {
        self.params.insert("mesalayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_mesalayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("mesalayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clifflayer(mut self, val: &str) -> Self {
        self.params.insert("clifflayer".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clifflayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("clifflayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_fold(mut self, val: bool) -> Self {
        self.params.insert("fold".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_fold_expr(mut self, expr: &str) -> Self {
        self.params.insert("fold".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldTerrace {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_terrace"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldTilesplitOrient {
    Auto = 0,
    XyPlane = 1,
    YzPlane = 2,
    ZxPlane = 3,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldTilesplit {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldTilesplit {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Int parameters ---
    pub fn with_tilenum(mut self, val: i32) -> Self {
        self.params.insert("tilenum".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_tilenum_expr(mut self, expr: &str) -> Self {
        self.params.insert("tilenum".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_voxelpad(mut self, val: i32) -> Self {
        self.params.insert("voxelpad".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_voxelpad_expr(mut self, expr: &str) -> Self {
        self.params.insert("voxelpad".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tileminoverlap(mut self, val: i32) -> Self {
        self.params.insert("tileminoverlap".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_tileminoverlap_expr(mut self, expr: &str) -> Self {
        self.params.insert("tileminoverlap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tilemaxoverlap(mut self, val: i32) -> Self {
        self.params.insert("tilemaxoverlap".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_tilemaxoverlap_expr(mut self, expr: &str) -> Self {
        self.params.insert("tilemaxoverlap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int2 parameters ---
    pub fn with_tilecount(mut self, val: [i32; 2]) -> Self {
        self.params.insert("tilecount".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_tilecount_expr(mut self, expr: &str) -> Self {
        self.params.insert("tilecount".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_orient(mut self, val: SopHeightfieldTilesplitOrient) -> Self {
        self.params.insert("orient".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_orient_expr(mut self, expr: &str) -> Self {
        self.params.insert("orient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_extracttile(mut self, val: bool) -> Self {
        self.params.insert("extracttile".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_extracttile_expr(mut self, expr: &str) -> Self {
        self.params.insert("extracttile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldTilesplit {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_tilesplit"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldVisualizeUpdatetinting {
    NoChange = 0,
    DefaultTinting = 1,
    CustomTinting = 2,
    RemoveTinting = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldVisualizeCdrampmode {
    NoRamp = 0,
    ClampedRamp = 1,
    PeriodicRamp = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldVisualizeCdpreset {
    Presets = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    Blackbody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldVisualizeUpdatematerial {
    NoChange = 0,
    CustomMaterial = 1,
    RemoveMaterial = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldVisualizeVisProjection {
    /// Y-Up
    YMinusUp = 0,
    /// Z-Up
    ZMinusUp = 1,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldVisualize {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldVisualize {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain"
    pub fn set_input_terrain<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain" and specifies the output index of the target node.
    pub fn set_input_terrain_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_computerange(mut self) -> Self {
        self.params.insert("computerange".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_computerange2(mut self) -> Self {
        self.params.insert("computerange2".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_vis_minelevation(mut self, val: f32) -> Self {
        self.params.insert("vis_minelevation".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_vis_minelevation_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_minelevation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_maxelevation(mut self, val: f32) -> Self {
        self.params.insert("vis_maxelevation".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_vis_maxelevation_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_maxelevation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_cdrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("cdrange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_cdrange_expr(mut self, expr: &str) -> Self {
        self.params.insert("cdrange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float4 parameters ---
    pub fn with_vis_color9(mut self, val: [f32; 4]) -> Self {
        self.params.insert("vis_color9".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_vis_color9_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_color9".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_color8(mut self, val: [f32; 4]) -> Self {
        self.params.insert("vis_color8".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_vis_color8_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_color8".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_color7(mut self, val: [f32; 4]) -> Self {
        self.params.insert("vis_color7".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_vis_color7_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_color7".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_color6(mut self, val: [f32; 4]) -> Self {
        self.params.insert("vis_color6".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_vis_color6_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_color6".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_color5(mut self, val: [f32; 4]) -> Self {
        self.params.insert("vis_color5".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_vis_color5_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_color5".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_color4(mut self, val: [f32; 4]) -> Self {
        self.params.insert("vis_color4".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_vis_color4_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_color4".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_color3(mut self, val: [f32; 4]) -> Self {
        self.params.insert("vis_color3".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_vis_color3_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_color3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_color2(mut self, val: [f32; 4]) -> Self {
        self.params.insert("vis_color2".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_vis_color2_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_color2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_color1(mut self, val: [f32; 4]) -> Self {
        self.params.insert("vis_color1".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_vis_color1_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_color1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_updatetinting(mut self, val: SopHeightfieldVisualizeUpdatetinting) -> Self {
        self.params.insert("updatetinting".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_updatetinting_expr(mut self, expr: &str) -> Self {
        self.params.insert("updatetinting".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cdrampmode(mut self, val: SopHeightfieldVisualizeCdrampmode) -> Self {
        self.params.insert("cdrampmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_cdrampmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("cdrampmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cdpreset(mut self, val: SopHeightfieldVisualizeCdpreset) -> Self {
        self.params.insert("cdpreset".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_cdpreset_expr(mut self, expr: &str) -> Self {
        self.params.insert("cdpreset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_updatematerial(mut self, val: SopHeightfieldVisualizeUpdatematerial) -> Self {
        self.params.insert("updatematerial".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_updatematerial_expr(mut self, expr: &str) -> Self {
        self.params.insert("updatematerial".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_projection(mut self, val: SopHeightfieldVisualizeVisProjection) -> Self {
        self.params.insert("vis_projection".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_vis_projection_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_projection".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_cdramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("cdramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_cdramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("cdramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_heightramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("vis_heightramp".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_vis_heightramp_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_heightramp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_heightvolume(mut self, val: &str) -> Self {
        self.params.insert("heightvolume".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_heightvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightvolume".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cdvolume(mut self, val: &str) -> Self {
        self.params.insert("cdvolume".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_cdvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert("cdvolume".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_layer9(mut self, val: &str) -> Self {
        self.params.insert("vis_layer9".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vis_layer9_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_layer9".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_layer8(mut self, val: &str) -> Self {
        self.params.insert("vis_layer8".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vis_layer8_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_layer8".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_layer7(mut self, val: &str) -> Self {
        self.params.insert("vis_layer7".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vis_layer7_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_layer7".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_layer6(mut self, val: &str) -> Self {
        self.params.insert("vis_layer6".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vis_layer6_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_layer6".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_layer5(mut self, val: &str) -> Self {
        self.params.insert("vis_layer5".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vis_layer5_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_layer5".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_layer4(mut self, val: &str) -> Self {
        self.params.insert("vis_layer4".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vis_layer4_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_layer4".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_layer3(mut self, val: &str) -> Self {
        self.params.insert("vis_layer3".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vis_layer3_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_layer3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_layer2(mut self, val: &str) -> Self {
        self.params.insert("vis_layer2".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vis_layer2_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_layer2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vis_layer1(mut self, val: &str) -> Self {
        self.params.insert("vis_layer1".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_vis_layer1_expr(mut self, expr: &str) -> Self {
        self.params.insert("vis_layer1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_cdrangeoverride(mut self, val: bool) -> Self {
        self.params.insert("cdrangeoverride".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_cdrangeoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert("cdrangeoverride".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldVisualize {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_visualize"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopHeightfieldXformOrient {
    Xy = 0,
    Yz = 1,
    Zx = 2,
}

#[derive(Debug, Clone)]
pub struct SopHeightfieldXform {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHeightfieldXform {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Terrain to Transform"
    pub fn set_input_terrain_to_transform<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Terrain to Transform" and specifies the output index of the target node.
    pub fn set_input_terrain_to_transform_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gridscale(mut self, val: f32) -> Self {
        self.params.insert("gridscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gridscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("gridscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightscale(mut self, val: f32) -> Self {
        self.params.insert("heightscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_heightscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightoffset(mut self, val: f32) -> Self {
        self.params.insert("heightoffset".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_heightoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightoffset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tx(mut self, val: f32) -> Self {
        self.params.insert("tx".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tx_expr(mut self, expr: &str) -> Self {
        self.params.insert("tx".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tz(mut self, val: f32) -> Self {
        self.params.insert("tz".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tz_expr(mut self, expr: &str) -> Self {
        self.params.insert("tz".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ry(mut self, val: f32) -> Self {
        self.params.insert("ry".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ry_expr(mut self, expr: &str) -> Self {
        self.params.insert("ry".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_orient(mut self, val: SopHeightfieldXformOrient) -> Self {
        self.params.insert("orient".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_orient_expr(mut self, expr: &str) -> Self {
        self.params.insert("orient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHeightfieldXform {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_xform"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}


#[derive(Debug, Clone)]
pub struct SopHole {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopHole {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_dist(mut self, val: f32) -> Self {
        self.params.insert("dist".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_dist_expr(mut self, expr: &str) -> Self {
        self.params.insert("dist".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_angle(mut self, val: f32) -> Self {
        self.params.insert("angle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert("angle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_break(mut self, val: bool) -> Self {
        self.params.insert("break".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_break_expr(mut self, expr: &str) -> Self {
        self.params.insert("break".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_snap(mut self, val: bool) -> Self {
        self.params.insert("snap".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_snap_expr(mut self, expr: &str) -> Self {
        self.params.insert("snap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_removeunusedpoints(mut self, val: bool) -> Self {
        self.params.insert("removeunusedpoints".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_removeunusedpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert("removeunusedpoints".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for SopHole {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "hole"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
