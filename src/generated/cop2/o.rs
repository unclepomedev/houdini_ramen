#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ObjnetXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ObjnetRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ObjnetPreXform {
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
pub enum Cop2ObjnetUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct Cop2Objnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Objnet {
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
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert("r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert("s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert("p".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_xord(mut self, val: Cop2ObjnetXord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: Cop2ObjnetRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pre_xform(mut self, val: Cop2ObjnetPreXform) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uparmtype(mut self, val: Cop2ObjnetUparmtype) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert("label1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert("label2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert("label3".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert("label4".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputobj".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert("visibleobjects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Objnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "objnet"
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
pub enum Cop2OutsideUnits {
    UvCoords = 0,
    Pixels = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OutsideWrap {
    Repeat = 0,
    Clamp = 1,
    Black = 2,
    Mirror = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OutsideMtype {
    Velocity = 0,
    Deformation = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OutsidePlanemerge {
    MergeAllPlanes = 0,
    OnlyKeepCommonPlanes = 1,
    /// Only Keep First Input's Planes
    OnlyKeepFirstInputSPlanes = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OutsideDepthmatch {
    PromoteToHighestBitDepth = 0,
    DemoteToLowestBitDepth = 1,
    /// Use the First Input's Bit Depth
    UseTheFirstInputSBitDepth = 2,
    ErrorIfBitDepthsDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OutsideRangematch {
    /// Trim To The First Input's Range
    TrimToTheFirstInputSRange = 0,
    /// Shift To The First Input's Range
    ShiftToTheFirstInputSRange = 1,
    ExtendSequenceToMaximumRange = 2,
    TrimToSmallestRange = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OutsideRatematch {
    /// Use The First Input's Frame Rate
    UseTheFirstInputSFrameRate = 0,
    UseTheHighestFrameRate = 1,
    UseTheLowestFrameRate = 2,
    ErrorIfTheFrameRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OutsideFramematch {
    UseNearestFrame = 0,
    UseTheClosestPreviousFrame = 1,
    UseTheClosestNextFrame = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OutsideMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OutsideFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OutsideFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OutsideFmenu {
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
pub struct Cop2Outside {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Outside {
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

    /// Connects to input 0: "Foreground"
    pub fn set_input_foreground<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Foreground" and specifies the output index of the target node.
    pub fn set_input_foreground_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Background"
    pub fn set_input_background<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Background" and specifies the output index of the target node.
    pub fn set_input_background_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Mask Input"
    pub fn set_input_mask_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_rz(mut self, val: f32) -> Self {
        self.params.insert("rz".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rz_expr(mut self, expr: &str) -> Self {
        self.params.insert("rz".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fgweight(mut self, val: f32) -> Self {
        self.params.insert("fgweight".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_fgweight_expr(mut self, expr: &str) -> Self {
        self.params.insert("fgweight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bgweight(mut self, val: f32) -> Self {
        self.params.insert("bgweight".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bgweight_expr(mut self, expr: &str) -> Self {
        self.params.insert("bgweight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mblur(mut self, val: f32) -> Self {
        self.params.insert("mblur".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_mblur_expr(mut self, expr: &str) -> Self {
        self.params.insert("mblur".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mbias(mut self, val: f32) -> Self {
        self.params.insert("mbias".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_mbias_expr(mut self, expr: &str) -> Self {
        self.params.insert("mbias".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert("effectamount".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert("effectamount".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_foutside(mut self, val: f32) -> Self {
        self.params.insert("foutside".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_foutside_expr(mut self, expr: &str) -> Self {
        self.params.insert("foutside".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_s(mut self, val: [f32; 2]) -> Self {
        self.params.insert("s".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert("s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_p(mut self, val: [f32; 2]) -> Self {
        self.params.insert("p".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert("p".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_frange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("frange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.params.insert("frange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fdropoff(mut self, val: [f32; 2]) -> Self {
        self.params.insert("fdropoff".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_fdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert("fdropoff".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_mseg(mut self, val: i32) -> Self {
        self.params.insert("mseg".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_mseg_expr(mut self, expr: &str) -> Self {
        self.params.insert("mseg".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert("scopergba".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert("scopergba".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_currange(mut self, val: i32) -> Self {
        self.params.insert("currange".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_currange_expr(mut self, expr: &str) -> Self {
        self.params.insert("currange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_units(mut self, val: Cop2OutsideUnits) -> Self {
        self.params.insert("units".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert("units".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_wrap(mut self, val: Cop2OutsideWrap) -> Self {
        self.params.insert("wrap".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_wrap_expr(mut self, expr: &str) -> Self {
        self.params.insert("wrap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mtype(mut self, val: Cop2OutsideMtype) -> Self {
        self.params.insert("mtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_mtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("mtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_planemerge(mut self, val: Cop2OutsidePlanemerge) -> Self {
        self.params.insert("planemerge".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_planemerge_expr(mut self, expr: &str) -> Self {
        self.params.insert("planemerge".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_depthmatch(mut self, val: Cop2OutsideDepthmatch) -> Self {
        self.params.insert("depthmatch".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_depthmatch_expr(mut self, expr: &str) -> Self {
        self.params.insert("depthmatch".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rangematch(mut self, val: Cop2OutsideRangematch) -> Self {
        self.params.insert("rangematch".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rangematch_expr(mut self, expr: &str) -> Self {
        self.params.insert("rangematch".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ratematch(mut self, val: Cop2OutsideRatematch) -> Self {
        self.params.insert("ratematch".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_ratematch_expr(mut self, expr: &str) -> Self {
        self.params.insert("ratematch".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_framematch(mut self, val: Cop2OutsideFramematch) -> Self {
        self.params.insert("framematch".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_framematch_expr(mut self, expr: &str) -> Self {
        self.params.insert("framematch".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maskinput(mut self, val: Cop2OutsideMaskinput) -> Self {
        self.params.insert("maskinput".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_maskinput_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskinput".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fscope(mut self, val: Cop2OutsideFscope) -> Self {
        self.params.insert("fscope".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_fscope_expr(mut self, expr: &str) -> Self {
        self.params.insert("fscope".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fdropfunc(mut self, val: Cop2OutsideFdropfunc) -> Self {
        self.params.insert("fdropfunc".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_fdropfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert("fdropfunc".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fmenu(mut self, val: Cop2OutsideFmenu) -> Self {
        self.params.insert("fmenu".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert("fmenu".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert("maskplane".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskplane".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pscope(mut self, val: &str) -> Self {
        self.params.insert("pscope".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pscope_expr(mut self, expr: &str) -> Self {
        self.params.insert("pscope".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flist(mut self, val: &str) -> Self {
        self.params.insert("flist".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_flist_expr(mut self, expr: &str) -> Self {
        self.params.insert("flist".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_useblur(mut self, val: bool) -> Self {
        self.params.insert("useblur".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useblur_expr(mut self, expr: &str) -> Self {
        self.params.insert("useblur".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert("maskresize".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskresize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_fautoadjust(mut self, val: bool) -> Self {
        self.params.insert("fautoadjust".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_fautoadjust_expr(mut self, expr: &str) -> Self {
        self.params.insert("fautoadjust".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Outside {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "outside"
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
pub enum Cop2OverUnits {
    UvCoords = 0,
    Pixels = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OverWrap {
    Repeat = 0,
    Clamp = 1,
    Black = 2,
    Mirror = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OverMtype {
    Velocity = 0,
    Deformation = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OverPlanemerge {
    MergeAllPlanes = 0,
    OnlyKeepCommonPlanes = 1,
    /// Only Keep First Input's Planes
    OnlyKeepFirstInputSPlanes = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OverDepthmatch {
    PromoteToHighestBitDepth = 0,
    DemoteToLowestBitDepth = 1,
    /// Use the First Input's Bit Depth
    UseTheFirstInputSBitDepth = 2,
    ErrorIfBitDepthsDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OverRangematch {
    /// Trim To The First Input's Range
    TrimToTheFirstInputSRange = 0,
    /// Shift To The First Input's Range
    ShiftToTheFirstInputSRange = 1,
    ExtendSequenceToMaximumRange = 2,
    TrimToSmallestRange = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OverRatematch {
    /// Use The First Input's Frame Rate
    UseTheFirstInputSFrameRate = 0,
    UseTheHighestFrameRate = 1,
    UseTheLowestFrameRate = 2,
    ErrorIfTheFrameRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OverFramematch {
    UseNearestFrame = 0,
    UseTheClosestPreviousFrame = 1,
    UseTheClosestNextFrame = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OverMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OverFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OverFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2OverFmenu {
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
pub struct Cop2Over {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Over {
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

    /// Connects to input 0: "Foreground"
    pub fn set_input_foreground<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Foreground" and specifies the output index of the target node.
    pub fn set_input_foreground_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Background"
    pub fn set_input_background<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Background" and specifies the output index of the target node.
    pub fn set_input_background_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Mask Input"
    pub fn set_input_mask_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Mask Input" and specifies the output index of the target node.
    pub fn set_input_mask_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_rz(mut self, val: f32) -> Self {
        self.params.insert("rz".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rz_expr(mut self, expr: &str) -> Self {
        self.params.insert("rz".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fgweight(mut self, val: f32) -> Self {
        self.params.insert("fgweight".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_fgweight_expr(mut self, expr: &str) -> Self {
        self.params.insert("fgweight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bgweight(mut self, val: f32) -> Self {
        self.params.insert("bgweight".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bgweight_expr(mut self, expr: &str) -> Self {
        self.params.insert("bgweight".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mblur(mut self, val: f32) -> Self {
        self.params.insert("mblur".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_mblur_expr(mut self, expr: &str) -> Self {
        self.params.insert("mblur".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mbias(mut self, val: f32) -> Self {
        self.params.insert("mbias".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_mbias_expr(mut self, expr: &str) -> Self {
        self.params.insert("mbias".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_effectamount(mut self, val: f32) -> Self {
        self.params.insert("effectamount".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_effectamount_expr(mut self, expr: &str) -> Self {
        self.params.insert("effectamount".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_foutside(mut self, val: f32) -> Self {
        self.params.insert("foutside".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_foutside_expr(mut self, expr: &str) -> Self {
        self.params.insert("foutside".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_s(mut self, val: [f32; 2]) -> Self {
        self.params.insert("s".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert("s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_p(mut self, val: [f32; 2]) -> Self {
        self.params.insert("p".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert("p".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_frange(mut self, val: [f32; 2]) -> Self {
        self.params.insert("frange".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.params.insert("frange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fdropoff(mut self, val: [f32; 2]) -> Self {
        self.params.insert("fdropoff".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_fdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert("fdropoff".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_mseg(mut self, val: i32) -> Self {
        self.params.insert("mseg".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_mseg_expr(mut self, expr: &str) -> Self {
        self.params.insert("mseg".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scopergba(mut self, val: i32) -> Self {
        self.params.insert("scopergba".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_scopergba_expr(mut self, expr: &str) -> Self {
        self.params.insert("scopergba".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_currange(mut self, val: i32) -> Self {
        self.params.insert("currange".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_currange_expr(mut self, expr: &str) -> Self {
        self.params.insert("currange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_units(mut self, val: Cop2OverUnits) -> Self {
        self.params.insert("units".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert("units".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_wrap(mut self, val: Cop2OverWrap) -> Self {
        self.params.insert("wrap".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_wrap_expr(mut self, expr: &str) -> Self {
        self.params.insert("wrap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mtype(mut self, val: Cop2OverMtype) -> Self {
        self.params.insert("mtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_mtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("mtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_planemerge(mut self, val: Cop2OverPlanemerge) -> Self {
        self.params.insert("planemerge".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_planemerge_expr(mut self, expr: &str) -> Self {
        self.params.insert("planemerge".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_depthmatch(mut self, val: Cop2OverDepthmatch) -> Self {
        self.params.insert("depthmatch".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_depthmatch_expr(mut self, expr: &str) -> Self {
        self.params.insert("depthmatch".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rangematch(mut self, val: Cop2OverRangematch) -> Self {
        self.params.insert("rangematch".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rangematch_expr(mut self, expr: &str) -> Self {
        self.params.insert("rangematch".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ratematch(mut self, val: Cop2OverRatematch) -> Self {
        self.params.insert("ratematch".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_ratematch_expr(mut self, expr: &str) -> Self {
        self.params.insert("ratematch".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_framematch(mut self, val: Cop2OverFramematch) -> Self {
        self.params.insert("framematch".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_framematch_expr(mut self, expr: &str) -> Self {
        self.params.insert("framematch".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maskinput(mut self, val: Cop2OverMaskinput) -> Self {
        self.params.insert("maskinput".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_maskinput_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskinput".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fscope(mut self, val: Cop2OverFscope) -> Self {
        self.params.insert("fscope".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_fscope_expr(mut self, expr: &str) -> Self {
        self.params.insert("fscope".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fdropfunc(mut self, val: Cop2OverFdropfunc) -> Self {
        self.params.insert("fdropfunc".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_fdropfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert("fdropfunc".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fmenu(mut self, val: Cop2OverFmenu) -> Self {
        self.params.insert("fmenu".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert("fmenu".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_maskplane(mut self, val: &str) -> Self {
        self.params.insert("maskplane".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_maskplane_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskplane".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pscope(mut self, val: &str) -> Self {
        self.params.insert("pscope".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pscope_expr(mut self, expr: &str) -> Self {
        self.params.insert("pscope".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flist(mut self, val: &str) -> Self {
        self.params.insert("flist".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_flist_expr(mut self, expr: &str) -> Self {
        self.params.insert("flist".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_useblur(mut self, val: bool) -> Self {
        self.params.insert("useblur".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useblur_expr(mut self, expr: &str) -> Self {
        self.params.insert("useblur".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maskresize(mut self, val: bool) -> Self {
        self.params.insert("maskresize".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_maskresize_expr(mut self, expr: &str) -> Self {
        self.params.insert("maskresize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_fautoadjust(mut self, val: bool) -> Self {
        self.params.insert("fautoadjust".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_fautoadjust_expr(mut self, expr: &str) -> Self {
        self.params.insert("fautoadjust".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Over {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "over"
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
