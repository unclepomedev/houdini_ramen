#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ScaleScaletype {
    ToResolution = 0,
    ToFraction = 1,
    ToAspectRatio = 2,
    /// To Second Input's Resolution
    ToSecondInputSResolution = 3,
    ToAFractionOfTheSecondInput = 4,
    /// To Second Input's Aspect Ratio
    ToSecondInputSAspectRatio = 5,
}

#[derive(Debug, Clone)]
pub struct Cop2Scale {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Scale {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Image to Scale"
    pub fn set_input_image_to_scale<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to Scale" and specifies the output index of the target node.
    pub fn set_input_image_to_scale_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Reference Image"
    pub fn set_input_reference_image<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Reference Image" and specifies the output index of the target node.
    pub fn set_input_reference_image_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_imageaspect(mut self, val: f32) -> Self {
        self.params.insert(
            "imageaspect".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_imageaspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imageaspect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_imagefract(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "imagefract".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_imagefract_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imagefract".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_imageres(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "imageres".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_imageres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imageres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_scaletype(mut self, val: Cop2ScaleScaletype) -> Self {
        self.params.insert(
            "scaletype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_scaletype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scaletype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_downfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "downfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_downfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "downfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "upfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_upfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_keepaspect(mut self, val: bool) -> Self {
        self.params.insert(
            "keepaspect".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepaspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepaspect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Scale {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "scale"
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
pub enum Cop2ScreenUnits {
    UvCoords = 0,
    Pixels = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ScreenWrap {
    Repeat = 0,
    Clamp = 1,
    Black = 2,
    Mirror = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ScreenMtype {
    Velocity = 0,
    Deformation = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ScreenPlanemerge {
    MergeAllPlanes = 0,
    OnlyKeepCommonPlanes = 1,
    /// Only Keep First Input's Planes
    OnlyKeepFirstInputSPlanes = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ScreenDepthmatch {
    PromoteToHighestBitDepth = 0,
    DemoteToLowestBitDepth = 1,
    /// Use the First Input's Bit Depth
    UseTheFirstInputSBitDepth = 2,
    ErrorIfBitDepthsDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ScreenRangematch {
    /// Trim To The First Input's Range
    TrimToTheFirstInputSRange = 0,
    /// Shift To The First Input's Range
    ShiftToTheFirstInputSRange = 1,
    ExtendSequenceToMaximumRange = 2,
    TrimToSmallestRange = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ScreenRatematch {
    /// Use The First Input's Frame Rate
    UseTheFirstInputSFrameRate = 0,
    UseTheHighestFrameRate = 1,
    UseTheLowestFrameRate = 2,
    ErrorIfTheFrameRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ScreenFramematch {
    UseNearestFrame = 0,
    UseTheClosestPreviousFrame = 1,
    UseTheClosestNextFrame = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ScreenMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ScreenFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ScreenFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ScreenFmenu {
    ScopeAllFrames = 0,
    ScopeCurrentFrame = 1,
    ScopeFromStartToCurrent = 2,
    ScopeFromCurrentToEnd = 3,
    UnscopeAllFrames = 4,
    UnscopeCurrentFrame = 5,
    UnscopeFromStartToCurrent = 6,
    UnscopeFromCurrentToEnd = 7,
}

#[derive(Debug, Clone)]
pub struct Cop2Screen {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Screen {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Foreground"
    pub fn set_input_foreground<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Foreground" and specifies the output index of the target node.
    pub fn set_input_foreground_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Background"
    pub fn set_input_background<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Background" and specifies the output index of the target node.
    pub fn set_input_background_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Mask Input"
    pub fn set_input_mask_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_rz(mut self, val: f32) -> Self {
        self.params
            .insert("rz".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fgweight(mut self, val: f32) -> Self {
        self.params.insert(
            "fgweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fgweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fgweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bgweight(mut self, val: f32) -> Self {
        self.params.insert(
            "bgweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bgweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bgweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mblur(mut self, val: f32) -> Self {
        self.params.insert(
            "mblur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mbias(mut self, val: f32) -> Self {
        self.params.insert(
            "mbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_foutside(mut self, val: f32) -> Self {
        self.params.insert(
            "foutside".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_foutside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foutside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_t(mut self, val: [f32; 2]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 2]) -> Self {
        self.params
            .insert("s".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 2]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_frange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "frange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fdropoff(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_fdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_mseg(mut self, val: i32) -> Self {
        self.params
            .insert("mseg".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_mseg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mseg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_currange(mut self, val: i32) -> Self {
        self.params.insert(
            "currange".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_currange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "currange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_units(mut self, val: Cop2ScreenUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wrap(mut self, val: Cop2ScreenWrap) -> Self {
        self.params.insert(
            "wrap".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_wrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mtype(mut self, val: Cop2ScreenMtype) -> Self {
        self.params.insert(
            "mtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_planemerge(mut self, val: Cop2ScreenPlanemerge) -> Self {
        self.params.insert(
            "planemerge".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_planemerge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planemerge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthmatch(mut self, val: Cop2ScreenDepthmatch) -> Self {
        self.params.insert(
            "depthmatch".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_depthmatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthmatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rangematch(mut self, val: Cop2ScreenRangematch) -> Self {
        self.params.insert(
            "rangematch".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rangematch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rangematch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ratematch(mut self, val: Cop2ScreenRatematch) -> Self {
        self.params.insert(
            "ratematch".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_ratematch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ratematch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_framematch(mut self, val: Cop2ScreenFramematch) -> Self {
        self.params.insert(
            "framematch".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_framematch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "framematch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskinput(mut self, val: Cop2ScreenMaskinput) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fscope(mut self, val: Cop2ScreenFscope) -> Self {
        self.params.insert(
            "fscope".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fscope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fdropfunc(mut self, val: Cop2ScreenFdropfunc) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fdropfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fmenu(mut self, val: Cop2ScreenFmenu) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_filter(mut self, val: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscope(mut self, val: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flist(mut self, val: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_flist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useblur(mut self, val: bool) -> Self {
        self.params.insert(
            "useblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fautoadjust(mut self, val: bool) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fautoadjust_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Screen {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "screen"
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
pub enum Cop2SequencePlanemerge {
    MergeAllPlanes = 0,
    OnlyKeepCommonPlanes = 1,
    /// Only Keep First Input's Planes
    OnlyKeepFirstInputSPlanes = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SequenceDepthmatch {
    PromoteToHighestBitDepth = 0,
    DemoteToLowestBitDepth = 1,
    /// Use the First Input's Bit Depth
    UseTheFirstInputSBitDepth = 2,
    ErrorIfBitDepthsDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SequenceResmatch {
    /// Use First Input's Resolution
    UseFirstInputSResolution = 0,
    SpecifyAResolution = 1,
    UseTheHighestResolution = 2,
    /// Use the Highest Resolution, First Input's Aspect
    UseTheHighestResolutionFirstInputSAspect = 3,
    UseTheLowestResolution = 4,
    /// Use the Lowest Resolution, First Input's Aspect
    UseTheLowestResolutionFirstInputSAspect = 5,
    ErrorIfResolutionsDiffer = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SequenceRatematch {
    /// Use The First Input's Frame Rate
    UseTheFirstInputSFrameRate = 0,
    UseTheHighestFrameRate = 1,
    UseTheLowestFrameRate = 2,
    ErrorIfTheFrameRatesDiffer = 3,
}

#[derive(Debug, Clone)]
pub struct Cop2Sequence {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl Cop2Sequence {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }

    // --- Int2 parameters ---
    pub fn with_size(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_planemerge(mut self, val: Cop2SequencePlanemerge) -> Self {
        self.params.insert(
            "planemerge".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_planemerge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planemerge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthmatch(mut self, val: Cop2SequenceDepthmatch) -> Self {
        self.params.insert(
            "depthmatch".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_depthmatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthmatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resmatch(mut self, val: Cop2SequenceResmatch) -> Self {
        self.params.insert(
            "resmatch".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_resmatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resmatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalefilter(mut self, val: i32) -> Self {
        self.params.insert(
            "scalefilter".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_scalefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalefilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ratematch(mut self, val: Cop2SequenceRatematch) -> Self {
        self.params.insert(
            "ratematch".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_ratematch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ratematch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_scaletosize(mut self, val: bool) -> Self {
        self.params.insert(
            "scaletosize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_scaletosize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scaletosize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Sequence {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sequence"
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
pub enum Cop2ShapeShape {
    /// Regular N-Sided
    RegularNMinusSided = 0,
    Star = 1,
    Circle = 2,
    Ring = 3,
    EquilateralTriangle = 4,
    IsoscelesTriangle = 5,
    RightTriangle = 6,
    Rectangle = 7,
    Parallelogram = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ShapeFill {
    Closed = 0,
    Hollow = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ShapePlanes {
    /// C, A (C:rgb A)
    CACRgbA = 0,
    /// C, A (C:rgb A:rgb)
    CACRgbARgb = 1,
    /// C (rgb)
    CRgb = 2,
    A = 3,
    /// A (rgb)
    ARgb = 4,
    M = 5,
    /// M (rgb)
    MRgb = 6,
    Z = 7,
    L = 8,
    /// B (uv)
    BUv = 9,
    /// P (xyz)
    PXyz = 10,
    /// N (xyz)
    NXyz = 11,
    /// V (xyz)
    VXyz = 12,
    /// Terrain: Height
    TerrainHeight = 13,
    None = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ShapeAddplanes {
    /// C, A (C:rgb A)
    CACRgbA = 0,
    /// C, A (C:rgb A:rgb)
    CACRgbARgb = 1,
    /// C (rgb)
    CRgb = 2,
    A = 3,
    /// A (rgb)
    ARgb = 4,
    M = 5,
    /// M (rgb)
    MRgb = 6,
    Z = 7,
    L = 8,
    /// B (uv)
    BUv = 9,
    /// P (xyz)
    PXyz = 10,
    /// N (xyz)
    NXyz = 11,
    /// V (xyz)
    VXyz = 12,
    /// Terrain: Height
    TerrainHeight = 13,
    None = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ShapeAddplaneop {
    Replace = 0,
    Rename = 1,
    Add = 2,
    Screen = 3,
    Subtract = 4,
    Multiply = 5,
    Min = 6,
    Max = 7,
    Average = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ShapeDepth {
    /// 8 Bit Integer
    N8BitInteger = 0,
    /// 16 Bit Integer
    N16BitInteger = 1,
    /// 32 Bit Integer
    N32BitInteger = 2,
    /// 16 Bit Floating Point
    N16BitFloatingPoint = 3,
    /// 32 Bit Floating Point
    N32BitFloatingPoint = 4,
    DefaultDepth = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ShapeInterlace {
    None = 0,
    HalfResInterlaced = 1,
    BlackInterlaced = 2,
    LineDoubled = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ShapeIdominance {
    OddFirst = 0,
    EvenFirst = 1,
    OddOnly = 2,
    EvenOnly = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ShapePreextend {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
    HoldNFrames = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ShapePostextend {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
    HoldNFrames = 4,
}

#[derive(Debug, Clone)]
pub struct Cop2Shape {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Shape {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Image to Add To"
    pub fn set_input_image_to_add_to<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to Add To" and specifies the output index of the target node.
    pub fn set_input_image_to_add_to_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask Input"
    pub fn set_input_mask_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_innerrad(mut self, val: f32) -> Self {
        self.params.insert(
            "innerrad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_innerrad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "innerrad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cornerangle(mut self, val: f32) -> Self {
        self.params.insert(
            "cornerangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cornerangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cornerangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lod(mut self, val: f32) -> Self {
        self.params.insert(
            "lod".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thickness(mut self, val: f32) -> Self {
        self.params.insert(
            "thickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_thickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate(mut self, val: f32) -> Self {
        self.params.insert(
            "rotate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aspect(mut self, val: f32) -> Self {
        self.params.insert(
            "aspect".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aspect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_sweepangles(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sweepangles".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sweepangles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sweepangles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_translate(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "translate".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_translate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "translate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pivot(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "pivot".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pivot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pivot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_color(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_numsides(mut self, val: i32) -> Self {
        self.params.insert(
            "numsides".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numsides_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numsides".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numpoints(mut self, val: i32) -> Self {
        self.params.insert(
            "numpoints".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthglobal(mut self, val: i32) -> Self {
        self.params.insert(
            "depthglobal".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_depthglobal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthglobal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_start(mut self, val: i32) -> Self {
        self.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_start_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_length(mut self, val: i32) -> Self {
        self.params.insert(
            "length".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_length_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "length".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prehold(mut self, val: i32) -> Self {
        self.params.insert(
            "prehold".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_prehold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prehold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_posthold(mut self, val: i32) -> Self {
        self.params.insert(
            "posthold".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_posthold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "posthold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_antialias(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "antialias".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_antialias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "antialias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_size(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bwpoints(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "bwpoints".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_bwpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bwpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_shape(mut self, val: Cop2ShapeShape) -> Self {
        self.params.insert(
            "shape".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shape_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shape".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill(mut self, val: Cop2ShapeFill) -> Self {
        self.params.insert(
            "fill".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fill_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fill".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sizemenu(mut self, val: i32) -> Self {
        self.params.insert(
            "sizemenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_sizemenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sizemenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_planes(mut self, val: Cop2ShapePlanes) -> Self {
        self.params.insert(
            "planes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_planes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addplanes(mut self, val: Cop2ShapeAddplanes) -> Self {
        self.params.insert(
            "addplanes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_addplanes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addplanes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addplaneop(mut self, val: Cop2ShapeAddplaneop) -> Self {
        self.params.insert(
            "addplaneop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_addplaneop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addplaneop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depth(mut self, val: Cop2ShapeDepth) -> Self {
        self.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_depth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "depthmenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_depthmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interlace(mut self, val: Cop2ShapeInterlace) -> Self {
        self.params.insert(
            "interlace".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_interlace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interlace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_idominance(mut self, val: Cop2ShapeIdominance) -> Self {
        self.params.insert(
            "idominance".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_idominance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "idominance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preextend(mut self, val: Cop2ShapePreextend) -> Self {
        self.params.insert(
            "preextend".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_preextend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preextend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postextend(mut self, val: Cop2ShapePostextend) -> Self {
        self.params.insert(
            "postextend".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_postextend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postextend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customplanes(mut self, val: &str) -> Self {
        self.params.insert(
            "customplanes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customplanes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customplanes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_openarc(mut self, val: bool) -> Self {
        self.params.insert(
            "openarc".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_openarc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "openarc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepaspect(mut self, val: bool) -> Self {
        self.params.insert(
            "keepaspect".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepaspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepaspect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fillout(mut self, val: bool) -> Self {
        self.params.insert(
            "fillout".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fillout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fillout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overridesize(mut self, val: bool) -> Self {
        self.params.insert(
            "overridesize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overridesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overridesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overrideaspect(mut self, val: bool) -> Self {
        self.params.insert(
            "overrideaspect".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overrideaspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overrideaspect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usebwpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "usebwpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebwpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usebwpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overriderange(mut self, val: bool) -> Self {
        self.params.insert(
            "overriderange".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overriderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overriderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_singleimage(mut self, val: bool) -> Self {
        self.params.insert(
            "singleimage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_singleimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "singleimage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Shape {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shape"
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
pub enum Cop2SharpenUnits {
    UvCoords = 0,
    Pixels = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SharpenSuppressfunc {
    SharpCutoff = 0,
    Linear = 1,
    EaseIn = 2,
    EaseOut = 3,
    EaseInEaseOut = 4,
    Gaussian = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SharpenMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SharpenFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SharpenFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SharpenFmenu {
    ScopeAllFrames = 0,
    ScopeCurrentFrame = 1,
    ScopeFromStartToCurrent = 2,
    ScopeFromCurrentToEnd = 3,
    UnscopeAllFrames = 4,
    UnscopeCurrentFrame = 5,
    UnscopeFromStartToCurrent = 6,
    UnscopeFromCurrentToEnd = 7,
}

#[derive(Debug, Clone)]
pub struct Cop2Sharpen {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Sharpen {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Image to Sharpen"
    pub fn set_input_image_to_sharpen<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to Sharpen" and specifies the output index of the target node.
    pub fn set_input_image_to_sharpen_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask Input"
    pub fn set_input_mask_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_effect(mut self, val: f32) -> Self {
        self.params.insert(
            "effect".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_threshold(mut self, val: f32) -> Self {
        self.params.insert(
            "threshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_threshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "threshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_suppressblend(mut self, val: f32) -> Self {
        self.params.insert(
            "suppressblend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_suppressblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "suppressblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_foutside(mut self, val: f32) -> Self {
        self.params.insert(
            "foutside".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_foutside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foutside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_ssize(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "ssize".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ssize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ssize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_frange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "frange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fdropoff(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_fdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_currange(mut self, val: i32) -> Self {
        self.params.insert(
            "currange".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_currange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "currange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_units(mut self, val: Cop2SharpenUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_suppressfunc(mut self, val: Cop2SharpenSuppressfunc) -> Self {
        self.params.insert(
            "suppressfunc".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_suppressfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "suppressfunc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskinput(mut self, val: Cop2SharpenMaskinput) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fscope(mut self, val: Cop2SharpenFscope) -> Self {
        self.params.insert(
            "fscope".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fscope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fdropfunc(mut self, val: Cop2SharpenFdropfunc) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fdropfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fmenu(mut self, val: Cop2SharpenFmenu) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscope(mut self, val: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flist(mut self, val: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_flist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_noisesuppress(mut self, val: bool) -> Self {
        self.params.insert(
            "noisesuppress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_noisesuppress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noisesuppress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_edgesonly(mut self, val: bool) -> Self {
        self.params.insert(
            "edgesonly".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_edgesonly_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "edgesonly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fautoadjust(mut self, val: bool) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fautoadjust_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Sharpen {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sharpen"
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
pub enum Cop2ShiftMethod {
    RelativeShift = 0,
    ShiftStartToShiftValue = 1,
    ShiftEndToShiftValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ShiftImethod {
    /// Align Start with 2nd Input's Start
    AlignStartWith2ndInputSStart = 0,
    /// Align End with 2nd Input's End
    AlignEndWith2ndInputSEnd = 1,
    ShiftToPrecede2ndInput = 2,
    ShiftToFollow2ndInput = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ShiftUnits {
    Frames = 0,
    Seconds = 1,
}

#[derive(Debug, Clone)]
pub struct Cop2Shift {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Shift {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Image to Shift"
    pub fn set_input_image_to_shift<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to Shift" and specifies the output index of the target node.
    pub fn set_input_image_to_shift_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Reference Image"
    pub fn set_input_reference_image<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Reference Image" and specifies the output index of the target node.
    pub fn set_input_reference_image_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_shiftval(mut self, val: f32) -> Self {
        self.params.insert(
            "shiftval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shiftval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shiftval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: Cop2ShiftMethod) -> Self {
        self.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imethod(mut self, val: Cop2ShiftImethod) -> Self {
        self.params.insert(
            "imethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_imethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: Cop2ShiftUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Shift {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shift"
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
pub struct Cop2Shopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Shopnet {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
}

impl crate::core::types::HoudiniNode for Cop2Shopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shopnet"
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
pub struct Cop2Shuffle {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Shuffle {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Sequence to Shuffle"
    pub fn set_input_sequence_to_shuffle<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sequence to Shuffle" and specifies the output index of the target node.
    pub fn set_input_sequence_to_shuffle_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_shuffle(mut self, val: &str) -> Self {
        self.params.insert(
            "shuffle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shuffle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shuffle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Shuffle {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shuffle"
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
pub enum Cop2SkycubePlanes {
    /// C, A (C:rgb A)
    CACRgbA = 0,
    /// C, A (C:rgb A:rgb)
    CACRgbARgb = 1,
    /// C (rgb)
    CRgb = 2,
    A = 3,
    /// A (rgb)
    ARgb = 4,
    M = 5,
    /// M (rgb)
    MRgb = 6,
    Z = 7,
    L = 8,
    /// B (uv)
    BUv = 9,
    /// P (xyz)
    PXyz = 10,
    /// N (xyz)
    NXyz = 11,
    /// V (xyz)
    VXyz = 12,
    /// Terrain: Height
    TerrainHeight = 13,
    None = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkycubeAddplanes {
    /// C, A (C:rgb A)
    CACRgbA = 0,
    /// C, A (C:rgb A:rgb)
    CACRgbARgb = 1,
    /// C (rgb)
    CRgb = 2,
    A = 3,
    /// A (rgb)
    ARgb = 4,
    M = 5,
    /// M (rgb)
    MRgb = 6,
    Z = 7,
    L = 8,
    /// B (uv)
    BUv = 9,
    /// P (xyz)
    PXyz = 10,
    /// N (xyz)
    NXyz = 11,
    /// V (xyz)
    VXyz = 12,
    /// Terrain: Height
    TerrainHeight = 13,
    None = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkycubeAddplaneop {
    Replace = 0,
    Rename = 1,
    Add = 2,
    Screen = 3,
    Subtract = 4,
    Multiply = 5,
    Min = 6,
    Max = 7,
    Average = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkycubeDepth {
    /// 8 Bit Integer
    N8BitInteger = 0,
    /// 16 Bit Integer
    N16BitInteger = 1,
    /// 32 Bit Integer
    N32BitInteger = 2,
    /// 16 Bit Floating Point
    N16BitFloatingPoint = 3,
    /// 32 Bit Floating Point
    N32BitFloatingPoint = 4,
    DefaultDepth = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkycubeInterlace {
    None = 0,
    HalfResInterlaced = 1,
    BlackInterlaced = 2,
    LineDoubled = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkycubeIdominance {
    OddFirst = 0,
    EvenFirst = 1,
    OddOnly = 2,
    EvenOnly = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkycubePreextend {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
    HoldNFrames = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkycubePostextend {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
    HoldNFrames = 4,
}

#[derive(Debug, Clone)]
pub struct Cop2Skycube {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Skycube {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_horizon_elevation(mut self, val: f32) -> Self {
        self.params.insert(
            "horizon_elevation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_horizon_elevation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "horizon_elevation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aspect(mut self, val: f32) -> Self {
        self.params.insert(
            "aspect".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aspect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sun_direction(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sun_direction".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sun_direction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_direction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ground_normal(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ground_normal".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ground_normal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ground_normal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ground_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ground_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ground_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ground_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_left(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "Left".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_left_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Left".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_top(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "Top".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_top_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Top".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bottom(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "Bottom".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_bottom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Bottom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_front(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "Front".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_front_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Front".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_back(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "Back".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_back_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Back".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_depthglobal(mut self, val: i32) -> Self {
        self.params.insert(
            "depthglobal".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_depthglobal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthglobal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_start(mut self, val: i32) -> Self {
        self.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_start_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_length(mut self, val: i32) -> Self {
        self.params.insert(
            "length".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_length_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "length".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prehold(mut self, val: i32) -> Self {
        self.params.insert(
            "prehold".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_prehold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prehold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_posthold(mut self, val: i32) -> Self {
        self.params.insert(
            "posthold".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_posthold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "posthold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_size(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bwpoints(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "bwpoints".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_bwpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bwpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sizemenu(mut self, val: i32) -> Self {
        self.params.insert(
            "sizemenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_sizemenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sizemenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_planes(mut self, val: Cop2SkycubePlanes) -> Self {
        self.params.insert(
            "planes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_planes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addplanes(mut self, val: Cop2SkycubeAddplanes) -> Self {
        self.params.insert(
            "addplanes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_addplanes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addplanes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addplaneop(mut self, val: Cop2SkycubeAddplaneop) -> Self {
        self.params.insert(
            "addplaneop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_addplaneop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addplaneop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depth(mut self, val: Cop2SkycubeDepth) -> Self {
        self.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_depth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "depthmenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_depthmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interlace(mut self, val: Cop2SkycubeInterlace) -> Self {
        self.params.insert(
            "interlace".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_interlace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interlace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_idominance(mut self, val: Cop2SkycubeIdominance) -> Self {
        self.params.insert(
            "idominance".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_idominance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "idominance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preextend(mut self, val: Cop2SkycubePreextend) -> Self {
        self.params.insert(
            "preextend".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_preextend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preextend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postextend(mut self, val: Cop2SkycubePostextend) -> Self {
        self.params.insert(
            "postextend".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_postextend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postextend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_sky_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "sky_ramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_sky_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_ramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_ground_type(mut self, val: &str) -> Self {
        self.params.insert(
            "ground_type".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ground_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ground_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customplanes(mut self, val: &str) -> Self {
        self.params.insert(
            "customplanes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customplanes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customplanes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overridesize(mut self, val: bool) -> Self {
        self.params.insert(
            "overridesize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overridesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overridesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overrideaspect(mut self, val: bool) -> Self {
        self.params.insert(
            "overrideaspect".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overrideaspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overrideaspect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usebwpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "usebwpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebwpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usebwpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overriderange(mut self, val: bool) -> Self {
        self.params.insert(
            "overriderange".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overriderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overriderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_singleimage(mut self, val: bool) -> Self {
        self.params.insert(
            "singleimage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_singleimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "singleimage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Skycube {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "skycube"
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
pub enum Cop2SkycubedaylightPlanes {
    /// C, A (C:rgb A)
    CACRgbA = 0,
    /// C, A (C:rgb A:rgb)
    CACRgbARgb = 1,
    /// C (rgb)
    CRgb = 2,
    A = 3,
    /// A (rgb)
    ARgb = 4,
    M = 5,
    /// M (rgb)
    MRgb = 6,
    Z = 7,
    L = 8,
    /// B (uv)
    BUv = 9,
    /// P (xyz)
    PXyz = 10,
    /// N (xyz)
    NXyz = 11,
    /// V (xyz)
    VXyz = 12,
    /// Terrain: Height
    TerrainHeight = 13,
    None = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkycubedaylightAddplanes {
    /// C, A (C:rgb A)
    CACRgbA = 0,
    /// C, A (C:rgb A:rgb)
    CACRgbARgb = 1,
    /// C (rgb)
    CRgb = 2,
    A = 3,
    /// A (rgb)
    ARgb = 4,
    M = 5,
    /// M (rgb)
    MRgb = 6,
    Z = 7,
    L = 8,
    /// B (uv)
    BUv = 9,
    /// P (xyz)
    PXyz = 10,
    /// N (xyz)
    NXyz = 11,
    /// V (xyz)
    VXyz = 12,
    /// Terrain: Height
    TerrainHeight = 13,
    None = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkycubedaylightAddplaneop {
    Replace = 0,
    Rename = 1,
    Add = 2,
    Screen = 3,
    Subtract = 4,
    Multiply = 5,
    Min = 6,
    Max = 7,
    Average = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkycubedaylightDepth {
    /// 8 Bit Integer
    N8BitInteger = 0,
    /// 16 Bit Integer
    N16BitInteger = 1,
    /// 32 Bit Integer
    N32BitInteger = 2,
    /// 16 Bit Floating Point
    N16BitFloatingPoint = 3,
    /// 32 Bit Floating Point
    N32BitFloatingPoint = 4,
    DefaultDepth = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkycubedaylightInterlace {
    None = 0,
    HalfResInterlaced = 1,
    BlackInterlaced = 2,
    LineDoubled = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkycubedaylightIdominance {
    OddFirst = 0,
    EvenFirst = 1,
    OddOnly = 2,
    EvenOnly = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkycubedaylightPreextend {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
    HoldNFrames = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkycubedaylightPostextend {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
    HoldNFrames = 4,
}

#[derive(Debug, Clone)]
pub struct Cop2Skycubedaylight {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Skycubedaylight {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_turbidity(mut self, val: f32) -> Self {
        self.params.insert(
            "turbidity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_turbidity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "turbidity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_brightness(mut self, val: f32) -> Self {
        self.params.insert(
            "brightness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_brightness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "brightness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aspect(mut self, val: f32) -> Self {
        self.params.insert(
            "aspect".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aspect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sun_direction(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sun_direction".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sun_direction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_direction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ground_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ground_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ground_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ground_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_left(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "Left".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_left_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Left".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_top(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "Top".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_top_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Top".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bottom(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "Bottom".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_bottom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Bottom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_front(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "Front".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_front_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Front".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_back(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "Back".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_back_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Back".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_depthglobal(mut self, val: i32) -> Self {
        self.params.insert(
            "depthglobal".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_depthglobal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthglobal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_start(mut self, val: i32) -> Self {
        self.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_start_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_length(mut self, val: i32) -> Self {
        self.params.insert(
            "length".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_length_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "length".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prehold(mut self, val: i32) -> Self {
        self.params.insert(
            "prehold".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_prehold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prehold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_posthold(mut self, val: i32) -> Self {
        self.params.insert(
            "posthold".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_posthold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "posthold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_size(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bwpoints(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "bwpoints".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_bwpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bwpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sizemenu(mut self, val: i32) -> Self {
        self.params.insert(
            "sizemenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_sizemenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sizemenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_planes(mut self, val: Cop2SkycubedaylightPlanes) -> Self {
        self.params.insert(
            "planes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_planes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addplanes(mut self, val: Cop2SkycubedaylightAddplanes) -> Self {
        self.params.insert(
            "addplanes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_addplanes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addplanes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addplaneop(mut self, val: Cop2SkycubedaylightAddplaneop) -> Self {
        self.params.insert(
            "addplaneop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_addplaneop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addplaneop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depth(mut self, val: Cop2SkycubedaylightDepth) -> Self {
        self.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_depth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "depthmenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_depthmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interlace(mut self, val: Cop2SkycubedaylightInterlace) -> Self {
        self.params.insert(
            "interlace".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_interlace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interlace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_idominance(mut self, val: Cop2SkycubedaylightIdominance) -> Self {
        self.params.insert(
            "idominance".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_idominance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "idominance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preextend(mut self, val: Cop2SkycubedaylightPreextend) -> Self {
        self.params.insert(
            "preextend".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_preextend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preextend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postextend(mut self, val: Cop2SkycubedaylightPostextend) -> Self {
        self.params.insert(
            "postextend".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_postextend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postextend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_ground_type(mut self, val: &str) -> Self {
        self.params.insert(
            "ground_type".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ground_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ground_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customplanes(mut self, val: &str) -> Self {
        self.params.insert(
            "customplanes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customplanes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customplanes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overridesize(mut self, val: bool) -> Self {
        self.params.insert(
            "overridesize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overridesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overridesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overrideaspect(mut self, val: bool) -> Self {
        self.params.insert(
            "overrideaspect".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overrideaspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overrideaspect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usebwpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "usebwpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebwpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usebwpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overriderange(mut self, val: bool) -> Self {
        self.params.insert(
            "overriderange".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overriderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overriderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_singleimage(mut self, val: bool) -> Self {
        self.params.insert(
            "singleimage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_singleimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "singleimage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Skycubedaylight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "skycubedaylight"
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
pub enum Cop2SkyenvmapGeoMonth {
    January = 0,
    February = 1,
    March = 2,
    April = 3,
    May = 4,
    June = 5,
    July = 6,
    August = 7,
    September = 8,
    October = 9,
    November = 10,
    December = 11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkyenvmapGeoHrSpec {
    Am = 0,
    Pm = 1,
    /// 24hr
    N24hr = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkyenvmapGeoTimeZone {
    /// Default (calculated from longitude)
    DefaultCalculatedFromLongitude = 0,
    /// UTC (GMT, WET)
    UtcGmtWet = 1,
    /// N (UTC - 1)
    NUtcMinus1 = 2,
    /// O (UTC - 2)
    OUtcMinus2 = 3,
    /// P (UTC - 3)
    PUtcMinus3 = 4,
    /// AST (UTC - 4)
    AstUtcMinus4 = 5,
    /// EST (UTC - 5)
    EstUtcMinus5 = 6,
    /// CST (UTC - 6)
    CstUtcMinus6 = 7,
    /// MST (UTC - 7)
    MstUtcMinus7 = 8,
    /// PST (UTC - 8)
    PstUtcMinus8 = 9,
    /// AKST (UTC - 9)
    AkstUtcMinus9 = 10,
    /// HAST (UTC - 10)
    HastUtcMinus10 = 11,
    /// X (UTC - 11)
    XUtcMinus11 = 12,
    /// M (UTC + 12)
    MUtcPlus12 = 13,
    /// L (UTC + 11)
    LUtcPlus11 = 14,
    /// EST (UTC + 10)
    EstUtcPlus10 = 15,
    /// I (UTC + 9)
    IUtcPlus9 = 16,
    /// WST (UTC + 8)
    WstUtcPlus8 = 17,
    /// CXT (UTC + 7)
    CxtUtcPlus7 = 18,
    /// F (UTC + 6)
    FUtcPlus6 = 19,
    /// E (UTC + 5)
    EUtcPlus5 = 20,
    /// D (UTC + 4)
    DUtcPlus4 = 21,
    /// MSK (UTC + 3)
    MskUtcPlus3 = 22,
    /// EET (UTC + 2)
    EetUtcPlus2 = 23,
    /// CET (UTC + 1)
    CetUtcPlus1 = 24,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SkyenvmapSkyColorSpec {
    Realistic = 0,
    Ramp = 1,
}

#[derive(Debug, Clone)]
pub struct Cop2Skyenvmap {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Skyenvmap {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn with_sun_azimuth(mut self, val: f32) -> Self {
        self.params.insert(
            "sun_azimuth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sun_azimuth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_azimuth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_elevation(mut self, val: f32) -> Self {
        self.params.insert(
            "sun_elevation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sun_elevation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_elevation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geo_latitude(mut self, val: f32) -> Self {
        self.params.insert(
            "geo_latitude".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_geo_latitude_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geo_latitude".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geo_longitude(mut self, val: f32) -> Self {
        self.params.insert(
            "geo_longitude".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_geo_longitude_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geo_longitude".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geo_doy(mut self, val: f32) -> Self {
        self.params.insert(
            "geo_doy".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_geo_doy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geo_doy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_turbidity(mut self, val: f32) -> Self {
        self.params.insert(
            "sky_turbidity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sky_turbidity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_turbidity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_brightness(mut self, val: f32) -> Self {
        self.params.insert(
            "sky_brightness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sky_brightness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_brightness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_horizon_elevation(mut self, val: f32) -> Self {
        self.params.insert(
            "horizon_elevation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_horizon_elevation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "horizon_elevation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_geo_time(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "geo_time".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_geo_time_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geo_time".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sun_dir_rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sun_dir_rotate".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sun_dir_rotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_dir_rotate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_dir_vector(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sun_dir_vector".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sun_dir_vector_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_dir_vector".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ground_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ground_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ground_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ground_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ground_normal(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ground_normal".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ground_normal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ground_normal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_resolution(mut self, val: i32) -> Self {
        self.params.insert(
            "resolution".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_resolution_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolution".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geo_day(mut self, val: i32) -> Self {
        self.params.insert(
            "geo_day".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_geo_day_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geo_day".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_geo_month(mut self, val: Cop2SkyenvmapGeoMonth) -> Self {
        self.params.insert(
            "geo_month".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_geo_month_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geo_month".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geo_hr_spec(mut self, val: Cop2SkyenvmapGeoHrSpec) -> Self {
        self.params.insert(
            "geo_hr_spec".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_geo_hr_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geo_hr_spec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geo_time_zone(mut self, val: Cop2SkyenvmapGeoTimeZone) -> Self {
        self.params.insert(
            "geo_time_zone".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_geo_time_zone_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geo_time_zone".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_color_spec(mut self, val: Cop2SkyenvmapSkyColorSpec) -> Self {
        self.params.insert(
            "sky_color_spec".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sky_color_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_color_spec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_sky_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "sky_ramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_sky_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_ramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sun_dir_spec(mut self, val: &str) -> Self {
        self.params.insert(
            "sun_dir_spec".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sun_dir_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_dir_spec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ground_type(mut self, val: &str) -> Self {
        self.params.insert(
            "ground_type".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ground_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ground_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_geo_daylight(mut self, val: bool) -> Self {
        self.params.insert(
            "geo_daylight".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geo_daylight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geo_daylight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Skyenvmap {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "skyenvmap"
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
#[allow(clippy::wrong_self_convention)]
pub trait Cop2SkyenvmapInnerExt {
    fn dir_angles_calc(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn dir_vector_calc(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn skycube(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn skycubedaylight(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> Cop2SkyenvmapInnerExt for crate::core::graph::InnerGraph<'a> {
    fn dir_angles_calc(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("dir_angles_calc")
    }
    fn dir_vector_calc(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("dir_vector_calc")
    }
    fn skycube(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("skycube")
    }
    fn skycubedaylight(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("skycubedaylight")
    }
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SnipOperation {
    SnipScopedFrames = 0,
    KeepScopedFrames = 1,
    UserDefinedFrameOrder = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SnipFmenu {
    ScopeAllFrames = 0,
    ScopeCurrentFrame = 1,
    ScopeStartToCurrent = 2,
    ScopeCurrentToEnd = 3,
    UnscopeAllFrames = 4,
    UnscopeCurrentFrame = 5,
    UnscopeStartToCurrent = 6,
    UnscopeCurrentToEnd = 7,
}

#[derive(Debug, Clone)]
pub struct Cop2Snip {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Snip {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Sequence to Snip"
    pub fn set_input_sequence_to_snip<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sequence to Snip" and specifies the output index of the target node.
    pub fn set_input_sequence_to_snip_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_currange(mut self, val: i32) -> Self {
        self.params.insert(
            "currange".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_currange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "currange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_operation(mut self, val: Cop2SnipOperation) -> Self {
        self.params.insert(
            "operation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_operation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "operation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fmenu(mut self, val: Cop2SnipFmenu) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_frames(mut self, val: &str) -> Self {
        self.params.insert(
            "frames".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_frames_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frames".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_order(mut self, val: &str) -> Self {
        self.params.insert(
            "order".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_order_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "order".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_adapt(mut self, val: bool) -> Self {
        self.params.insert(
            "adapt".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adapt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adapt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Snip {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "snip"
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
pub enum Cop2SopimportPlanes {
    /// C, A (C:rgb A)
    CACRgbA = 0,
    /// C, A (C:rgb A:rgb)
    CACRgbARgb = 1,
    /// C (rgb)
    CRgb = 2,
    A = 3,
    /// A (rgb)
    ARgb = 4,
    M = 5,
    /// M (rgb)
    MRgb = 6,
    Z = 7,
    L = 8,
    /// B (uv)
    BUv = 9,
    /// P (xyz)
    PXyz = 10,
    /// N (xyz)
    NXyz = 11,
    /// V (xyz)
    VXyz = 12,
    /// Terrain: Height
    TerrainHeight = 13,
    None = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SopimportAddplanes {
    /// C, A (C:rgb A)
    CACRgbA = 0,
    /// C, A (C:rgb A:rgb)
    CACRgbARgb = 1,
    /// C (rgb)
    CRgb = 2,
    A = 3,
    /// A (rgb)
    ARgb = 4,
    M = 5,
    /// M (rgb)
    MRgb = 6,
    Z = 7,
    L = 8,
    /// B (uv)
    BUv = 9,
    /// P (xyz)
    PXyz = 10,
    /// N (xyz)
    NXyz = 11,
    /// V (xyz)
    VXyz = 12,
    /// Terrain: Height
    TerrainHeight = 13,
    None = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SopimportAddplaneop {
    Replace = 0,
    Rename = 1,
    Add = 2,
    Screen = 3,
    Subtract = 4,
    Multiply = 5,
    Min = 6,
    Max = 7,
    Average = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SopimportDepth {
    /// 8 Bit Integer
    N8BitInteger = 0,
    /// 16 Bit Integer
    N16BitInteger = 1,
    /// 32 Bit Integer
    N32BitInteger = 2,
    /// 16 Bit Floating Point
    N16BitFloatingPoint = 3,
    /// 32 Bit Floating Point
    N32BitFloatingPoint = 4,
    DefaultDepth = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SopimportInterlace {
    None = 0,
    HalfResInterlaced = 1,
    BlackInterlaced = 2,
    LineDoubled = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SopimportIdominance {
    OddFirst = 0,
    EvenFirst = 1,
    OddOnly = 2,
    EvenOnly = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SopimportPreextend {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
    HoldNFrames = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SopimportPostextend {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
    HoldNFrames = 4,
}

#[derive(Debug, Clone)]
pub struct Cop2Sopimport {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Sopimport {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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

    // --- Button parameters ---
    pub fn trigger_setres(mut self) -> Self {
        self.params
            .insert("setres".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_setplanes(mut self) -> Self {
        self.params.insert(
            "setplanes".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aspect(mut self, val: f32) -> Self {
        self.params.insert(
            "aspect".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aspect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_depthglobal(mut self, val: i32) -> Self {
        self.params.insert(
            "depthglobal".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_depthglobal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthglobal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_start(mut self, val: i32) -> Self {
        self.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_start_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_length(mut self, val: i32) -> Self {
        self.params.insert(
            "length".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_length_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "length".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prehold(mut self, val: i32) -> Self {
        self.params.insert(
            "prehold".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_prehold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prehold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_posthold(mut self, val: i32) -> Self {
        self.params.insert(
            "posthold".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_posthold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "posthold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_remaprange(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "remaprange".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_remaprange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remaprange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_size(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bwpoints(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "bwpoints".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_bwpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bwpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sizemenu(mut self, val: i32) -> Self {
        self.params.insert(
            "sizemenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_sizemenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sizemenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_planes(mut self, val: Cop2SopimportPlanes) -> Self {
        self.params.insert(
            "planes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_planes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addplanes(mut self, val: Cop2SopimportAddplanes) -> Self {
        self.params.insert(
            "addplanes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_addplanes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addplanes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addplaneop(mut self, val: Cop2SopimportAddplaneop) -> Self {
        self.params.insert(
            "addplaneop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_addplaneop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addplaneop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depth(mut self, val: Cop2SopimportDepth) -> Self {
        self.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_depth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "depthmenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_depthmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interlace(mut self, val: Cop2SopimportInterlace) -> Self {
        self.params.insert(
            "interlace".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_interlace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interlace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_idominance(mut self, val: Cop2SopimportIdominance) -> Self {
        self.params.insert(
            "idominance".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_idominance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "idominance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preextend(mut self, val: Cop2SopimportPreextend) -> Self {
        self.params.insert(
            "preextend".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_preextend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preextend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postextend(mut self, val: Cop2SopimportPostextend) -> Self {
        self.params.insert(
            "postextend".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_postextend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postextend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importmask(mut self, val: &str) -> Self {
        self.params.insert(
            "importmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_importmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customplanes(mut self, val: &str) -> Self {
        self.params.insert(
            "customplanes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customplanes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customplanes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_remap(mut self, val: bool) -> Self {
        self.params.insert(
            "remap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overridesize(mut self, val: bool) -> Self {
        self.params.insert(
            "overridesize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overridesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overridesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overrideaspect(mut self, val: bool) -> Self {
        self.params.insert(
            "overrideaspect".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overrideaspect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overrideaspect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usebwpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "usebwpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebwpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usebwpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overriderange(mut self, val: bool) -> Self {
        self.params.insert(
            "overriderange".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overriderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overriderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_singleimage(mut self, val: bool) -> Self {
        self.params.insert(
            "singleimage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_singleimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "singleimage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Sopimport {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sopimport"
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
pub enum Cop2SopnetXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SopnetRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SopnetPreXform {
    CleanTransform = 0,
    CleanTranslates = 1,
    CleanRotates = 2,
    CleanScales = 3,
    /// Extract Pre-transform
    ExtractPreMinusTransform = 4,
    /// Reset Pre-transform
    ResetPreMinusTransform = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SopnetUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct Cop2Sopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Sopnet {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert(
            "bank".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bank".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_shop_materialopts(mut self, val: i32) -> Self {
        self.params.insert(
            "shop_materialopts".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_shop_materialopts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_materialopts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: Cop2SopnetXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: Cop2SopnetRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pre_xform(mut self, val: Cop2SopnetPreXform) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: Cop2SopnetUparmtype) -> Self {
        self.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath(mut self, val: &str) -> Self {
        self.params.insert(
            "shop_materialpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_materialpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showlopstage(mut self, val: &str) -> Self {
        self.params.insert(
            "showlopstage".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_showlopstage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showlopstage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vport_shadeopen(mut self, val: bool) -> Self {
        self.params.insert(
            "vport_shadeopen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_shadeopen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_shadeopen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vport_displayassubdiv(mut self, val: bool) -> Self {
        self.params.insert(
            "vport_displayassubdiv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_displayassubdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_displayassubdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Sopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sopnet"
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
pub enum Cop2StreakFalloff {
    None = 0,
    Linear = 1,
    Gaussian = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2StreakMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2StreakFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2StreakFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2StreakFmenu {
    ScopeAllFrames = 0,
    ScopeCurrentFrame = 1,
    ScopeFromStartToCurrent = 2,
    ScopeFromCurrentToEnd = 3,
    UnscopeAllFrames = 4,
    UnscopeCurrentFrame = 5,
    UnscopeFromStartToCurrent = 6,
    UnscopeFromCurrentToEnd = 7,
}

#[derive(Debug, Clone)]
pub struct Cop2Streak {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Streak {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Image to Streak"
    pub fn set_input_image_to_streak<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to Streak" and specifies the output index of the target node.
    pub fn set_input_image_to_streak_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Mask Input"
    pub fn set_input_mask_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_size(mut self, val: f32) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rot(mut self, val: f32) -> Self {
        self.params.insert(
            "rot".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bias(mut self, val: f32) -> Self {
        self.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffamount(mut self, val: f32) -> Self {
        self.params.insert(
            "falloffamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_falloffamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "falloffamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_foutside(mut self, val: f32) -> Self {
        self.params.insert(
            "foutside".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_foutside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foutside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_frange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "frange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fdropoff(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_fdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_currange(mut self, val: i32) -> Self {
        self.params.insert(
            "currange".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_currange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "currange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_falloff(mut self, val: Cop2StreakFalloff) -> Self {
        self.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_falloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskinput(mut self, val: Cop2StreakMaskinput) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fscope(mut self, val: Cop2StreakFscope) -> Self {
        self.params.insert(
            "fscope".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fscope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fdropfunc(mut self, val: Cop2StreakFdropfunc) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fdropfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fmenu(mut self, val: Cop2StreakFmenu) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscope(mut self, val: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flist(mut self, val: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_flist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fautoadjust(mut self, val: bool) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fautoadjust_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Streak {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "streak"
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
pub struct Cop2Subnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Subnet {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2"
    pub fn set_input_sub_network_input_2<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Sub-Network Input #3"
    pub fn set_input_sub_network_input_3<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Sub-Network Input #3" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Sub-Network Input #4"
    pub fn set_input_sub_network_input_4<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Sub-Network Input #4" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert(
            "label1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert(
            "label2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert(
            "label3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert(
            "label4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Subnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "subnet"
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
pub enum Cop2SubtractUnits {
    UvCoords = 0,
    Pixels = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SubtractWrap {
    Repeat = 0,
    Clamp = 1,
    Black = 2,
    Mirror = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SubtractMtype {
    Velocity = 0,
    Deformation = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SubtractPlanemerge {
    MergeAllPlanes = 0,
    OnlyKeepCommonPlanes = 1,
    /// Only Keep First Input's Planes
    OnlyKeepFirstInputSPlanes = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SubtractDepthmatch {
    PromoteToHighestBitDepth = 0,
    DemoteToLowestBitDepth = 1,
    /// Use the First Input's Bit Depth
    UseTheFirstInputSBitDepth = 2,
    ErrorIfBitDepthsDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SubtractRangematch {
    /// Trim To The First Input's Range
    TrimToTheFirstInputSRange = 0,
    /// Shift To The First Input's Range
    ShiftToTheFirstInputSRange = 1,
    ExtendSequenceToMaximumRange = 2,
    TrimToSmallestRange = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SubtractRatematch {
    /// Use The First Input's Frame Rate
    UseTheFirstInputSFrameRate = 0,
    UseTheHighestFrameRate = 1,
    UseTheLowestFrameRate = 2,
    ErrorIfTheFrameRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SubtractFramematch {
    UseNearestFrame = 0,
    UseTheClosestPreviousFrame = 1,
    UseTheClosestNextFrame = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SubtractMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SubtractFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SubtractFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SubtractFmenu {
    ScopeAllFrames = 0,
    ScopeCurrentFrame = 1,
    ScopeFromStartToCurrent = 2,
    ScopeFromCurrentToEnd = 3,
    UnscopeAllFrames = 4,
    UnscopeCurrentFrame = 5,
    UnscopeFromStartToCurrent = 6,
    UnscopeFromCurrentToEnd = 7,
}

#[derive(Debug, Clone)]
pub struct Cop2Subtract {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Subtract {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Foreground"
    pub fn set_input_foreground<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Foreground" and specifies the output index of the target node.
    pub fn set_input_foreground_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Background"
    pub fn set_input_background<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Background" and specifies the output index of the target node.
    pub fn set_input_background_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Mask Input"
    pub fn set_input_mask_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_rz(mut self, val: f32) -> Self {
        self.params
            .insert("rz".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fgweight(mut self, val: f32) -> Self {
        self.params.insert(
            "fgweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fgweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fgweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bgweight(mut self, val: f32) -> Self {
        self.params.insert(
            "bgweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bgweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bgweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mblur(mut self, val: f32) -> Self {
        self.params.insert(
            "mblur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mbias(mut self, val: f32) -> Self {
        self.params.insert(
            "mbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effectamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_foutside(mut self, val: f32) -> Self {
        self.params.insert(
            "foutside".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_foutside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foutside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_t(mut self, val: [f32; 2]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 2]) -> Self {
        self.params
            .insert("s".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 2]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_frange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "frange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fdropoff(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_fdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_mseg(mut self, val: i32) -> Self {
        self.params
            .insert("mseg".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_mseg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mseg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scopergba".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_currange(mut self, val: i32) -> Self {
        self.params.insert(
            "currange".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_currange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "currange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_units(mut self, val: Cop2SubtractUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wrap(mut self, val: Cop2SubtractWrap) -> Self {
        self.params.insert(
            "wrap".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_wrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mtype(mut self, val: Cop2SubtractMtype) -> Self {
        self.params.insert(
            "mtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_planemerge(mut self, val: Cop2SubtractPlanemerge) -> Self {
        self.params.insert(
            "planemerge".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_planemerge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planemerge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthmatch(mut self, val: Cop2SubtractDepthmatch) -> Self {
        self.params.insert(
            "depthmatch".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_depthmatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthmatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rangematch(mut self, val: Cop2SubtractRangematch) -> Self {
        self.params.insert(
            "rangematch".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rangematch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rangematch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ratematch(mut self, val: Cop2SubtractRatematch) -> Self {
        self.params.insert(
            "ratematch".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_ratematch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ratematch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_framematch(mut self, val: Cop2SubtractFramematch) -> Self {
        self.params.insert(
            "framematch".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_framematch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "framematch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskinput(mut self, val: Cop2SubtractMaskinput) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fscope(mut self, val: Cop2SubtractFscope) -> Self {
        self.params.insert(
            "fscope".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fscope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fdropfunc(mut self, val: Cop2SubtractFdropfunc) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fdropfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fdropfunc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fmenu(mut self, val: Cop2SubtractFmenu) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_filter(mut self, val: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscope(mut self, val: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flist(mut self, val: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_flist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useblur(mut self, val: bool) -> Self {
        self.params.insert(
            "useblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskresize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maskinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fautoadjust(mut self, val: bool) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fautoadjust_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fautoadjust".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Subtract {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "subtract"
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
pub enum Cop2SwapSwapop1 {
    AssignChannelOnly = 0,
    SwapChannels = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SwapSwapop2 {
    AssignChannelOnly = 0,
    SwapChannels = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SwapSwapop3 {
    AssignChannelOnly = 0,
    SwapChannels = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2SwapSwapop4 {
    AssignChannelOnly = 0,
    SwapChannels = 1,
}

#[derive(Debug, Clone)]
pub struct Cop2Swap {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Swap {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Images to Select From"
    pub fn set_input_images_to_select_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Images to Select From" and specifies the output index of the target node.
    pub fn set_input_images_to_select_from_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Menu parameters ---
    pub fn with_swapop1(mut self, val: Cop2SwapSwapop1) -> Self {
        self.params.insert(
            "swapop1".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_swapop1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapop1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapop2(mut self, val: Cop2SwapSwapop2) -> Self {
        self.params.insert(
            "swapop2".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_swapop2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapop2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapop3(mut self, val: Cop2SwapSwapop3) -> Self {
        self.params.insert(
            "swapop3".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_swapop3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapop3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapop4(mut self, val: Cop2SwapSwapop4) -> Self {
        self.params.insert(
            "swapop4".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_swapop4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapop4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_source1(mut self, val: &str) -> Self {
        self.params.insert(
            "source1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_source1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_target1(mut self, val: &str) -> Self {
        self.params.insert(
            "target1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_target1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "target1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source2(mut self, val: &str) -> Self {
        self.params.insert(
            "source2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_source2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_target2(mut self, val: &str) -> Self {
        self.params.insert(
            "target2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_target2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "target2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source3(mut self, val: &str) -> Self {
        self.params.insert(
            "source3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_source3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_target3(mut self, val: &str) -> Self {
        self.params.insert(
            "target3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_target3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "target3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source4(mut self, val: &str) -> Self {
        self.params.insert(
            "source4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_source4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_target4(mut self, val: &str) -> Self {
        self.params.insert(
            "target4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_target4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "target4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Swap {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "swap"
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
pub struct Cop2Switch {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl Cop2Switch {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }

    // --- Int parameters ---
    pub fn with_index(mut self, val: i32) -> Self {
        self.params.insert(
            "index".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_index_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "index".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Switch {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "switch"
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
pub struct Cop2Switchalpha {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Switchalpha {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Image to Modify"
    pub fn set_input_image_to_modify<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to Modify" and specifies the output index of the target node.
    pub fn set_input_image_to_modify_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Alpha Matte"
    pub fn set_input_alpha_matte<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Alpha Matte" and specifies the output index of the target node.
    pub fn set_input_alpha_matte_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_premultscope(mut self, val: &str) -> Self {
        self.params.insert(
            "premultscope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_premultscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "premultscope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_premult(mut self, val: bool) -> Self {
        self.params.insert(
            "premult".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_premult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "premult".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Switchalpha {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "switchalpha"
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
#[allow(clippy::wrong_self_convention)]
pub trait Cop2SwitchalphaInnerExt {
    fn channelcopy1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn premultiply1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn premultiply2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch2(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> Cop2SwitchalphaInnerExt for crate::core::graph::InnerGraph<'a> {
    fn channelcopy1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("channelcopy1")
    }
    fn premultiply1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("premultiply1")
    }
    fn premultiply2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("premultiply2")
    }
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch1")
    }
    fn switch2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch2")
    }
}
