#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EdgeType {
    Prewitt = 0,
    Sobel = 1,
    /// Marr-Hildreth
    MarrMinusHildreth = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EdgeUnits {
    UvCoords = 0,
    Pixels = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EdgeMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EdgeFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EdgeFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EdgeFmenu {
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
pub struct Cop2Edge {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Edge {
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

    /// Connects to input 0: "Image to Edge Detect"
    pub fn set_input_image_to_edge_detect<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to Edge Detect" and specifies the output index of the target node.
    pub fn set_input_image_to_edge_detect_from<N: crate::core::types::HoudiniNode>(
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
    pub fn with_featuresize(mut self, val: f32) -> Self {
        self.params.insert(
            "featuresize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_featuresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "featuresize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noisethresh(mut self, val: f32) -> Self {
        self.params.insert(
            "noisethresh".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_noisethresh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noisethresh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_connectthresh(mut self, val: f32) -> Self {
        self.params.insert(
            "connectthresh".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_connectthresh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "connectthresh".to_string(),
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
    pub fn with_size(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Float2(val),
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
    pub fn with_connectpass(mut self, val: i32) -> Self {
        self.params.insert(
            "connectpass".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_connectpass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "connectpass".to_string(),
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
    pub fn with_type(mut self, val: Cop2EdgeType) -> Self {
        self.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: Cop2EdgeUnits) -> Self {
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
    pub fn with_maskinput(mut self, val: Cop2EdgeMaskinput) -> Self {
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
    pub fn with_fscope(mut self, val: Cop2EdgeFscope) -> Self {
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
    pub fn with_fdropfunc(mut self, val: Cop2EdgeFdropfunc) -> Self {
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
    pub fn with_fmenu(mut self, val: Cop2EdgeFmenu) -> Self {
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
    pub fn with_antialias(mut self, val: bool) -> Self {
        self.params.insert(
            "antialias".to_string(),
            crate::core::types::ParamValue::Toggle(val),
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

impl crate::core::types::HoudiniNode for Cop2Edge {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "edge"
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
pub struct Cop2Edgeblur {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Edgeblur {
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

    /// Connects to input 0: "Edges to Blur"
    pub fn set_input_edges_to_blur<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Edges to Blur" and specifies the output index of the target node.
    pub fn set_input_edges_to_blur_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_blursize(mut self, val: f32) -> Self {
        self.params.insert(
            "blursize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blursize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blursize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_soften(mut self, val: f32) -> Self {
        self.params.insert(
            "soften".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_soften_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soften".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Edgeblur {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "edgeblur"
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
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait Cop2EdgeblurInnerExt {
    fn bedge_blur(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn convert_to_fp(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn edge_detect_alpha(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn mult_effect(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn soften_matte(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> Cop2EdgeblurInnerExt for crate::core::graph::InnerGraph<'a> {
    fn bedge_blur(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("bedge_blur")
    }
    fn convert_to_fp(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("convert_to_fp")
    }
    fn edge_detect_alpha(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("edge_detect_Alpha")
    }
    fn mult_effect(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("mult_effect")
    }
    fn soften_matte(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("soften_matte")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EmbossSpecmodel {
    Phong = 0,
    Blinn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EmbossMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EmbossFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EmbossFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EmbossFmenu {
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
pub struct Cop2Emboss {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Emboss {
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

    /// Connects to input 0: "Image to Emboss"
    pub fn set_input_image_to_emboss<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to Emboss" and specifies the output index of the target node.
    pub fn set_input_image_to_emboss_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Bump Map (optional)"
    pub fn set_input_bump_map_optional<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Bump Map (optional)" and specifies the output index of the target node.
    pub fn set_input_bump_map_optional_from<N: crate::core::types::HoudiniNode>(
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
    pub fn with_diffdimmer(mut self, val: f32) -> Self {
        self.params.insert(
            "diffdimmer".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diffdimmer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffdimmer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specdimmer(mut self, val: f32) -> Self {
        self.params.insert(
            "specdimmer".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_specdimmer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specdimmer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bumpscale(mut self, val: f32) -> Self {
        self.params.insert(
            "bumpscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bumpscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bumpscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specexp(mut self, val: f32) -> Self {
        self.params.insert(
            "specexp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_specexp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specexp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specrough(mut self, val: f32) -> Self {
        self.params.insert(
            "specrough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_specrough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specrough".to_string(),
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

    // --- Float3 parameters ---
    pub fn with_lightpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lightpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lightpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_diff(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "diff".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_diff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "spec".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_specmodel(mut self, val: Cop2EmbossSpecmodel) -> Self {
        self.params.insert(
            "specmodel".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_specmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specmodel".to_string(),
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
    pub fn with_maskinput(mut self, val: Cop2EmbossMaskinput) -> Self {
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
    pub fn with_fscope(mut self, val: Cop2EmbossFscope) -> Self {
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
    pub fn with_fdropfunc(mut self, val: Cop2EmbossFdropfunc) -> Self {
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
    pub fn with_fmenu(mut self, val: Cop2EmbossFmenu) -> Self {
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
    pub fn with_bumpplane(mut self, val: &str) -> Self {
        self.params.insert(
            "bumpplane".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bumpplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bumpplane".to_string(),
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

impl crate::core::types::HoudiniNode for Cop2Emboss {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "emboss"
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
pub enum Cop2EnvMaptype {
    Spherical = 0,
    /// Cube (Deep Raster)
    CubeDeepRaster = 1,
    /// Cube (Vertical Cross)
    CubeVerticalCross = 2,
    /// Cube (Horizontal Cross)
    CubeHorizontalCross = 3,
    /// Cube (Strip)
    CubeStrip = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EnvSource {
    NormalMap = 0,
    BumpMap = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EnvMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EnvFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EnvFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EnvFmenu {
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
pub struct Cop2Env {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Env {
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

    /// Connects to input 0: "Color & Bump/Normal"
    pub fn set_input_color_bump_normal<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Color & Bump/Normal" and specifies the output index of the target node.
    pub fn set_input_color_bump_normal_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Environment Map"
    pub fn set_input_environment_map<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Environment Map" and specifies the output index of the target node.
    pub fn set_input_environment_map_from<N: crate::core::types::HoudiniNode>(
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
    pub fn with_reflect(mut self, val: f32) -> Self {
        self.params.insert(
            "reflect".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ambient(mut self, val: f32) -> Self {
        self.params.insert(
            "ambient".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ambient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ambient".to_string(),
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

    // --- Float3 parameters ---
    pub fn with_rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate".to_string(),
            crate::core::types::ParamValue::Float3(val),
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

    // --- Int parameters ---
    pub fn with_maptype(mut self, val: Cop2EnvMaptype) -> Self {
        self.params.insert(
            "maptype".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_maptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cubex(mut self, val: i32) -> Self {
        self.params.insert(
            "cubex".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cubex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cubex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cubey(mut self, val: i32) -> Self {
        self.params.insert(
            "cubey".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cubey_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cubey".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source(mut self, val: Cop2EnvSource) -> Self {
        self.params.insert(
            "source".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_source_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source".to_string(),
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
    pub fn with_maskinput(mut self, val: Cop2EnvMaskinput) -> Self {
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
    pub fn with_fscope(mut self, val: Cop2EnvFscope) -> Self {
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
    pub fn with_fdropfunc(mut self, val: Cop2EnvFdropfunc) -> Self {
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
    pub fn with_fmenu(mut self, val: Cop2EnvFmenu) -> Self {
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

impl crate::core::types::HoudiniNode for Cop2Env {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "env"
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
pub enum Cop2EqualizeOp {
    /// Stretch Range to Black/White
    StretchRangeToBlackWhite = 0,
    MoveMinimumValueToBlack = 1,
    MoveMaximumValueToWhite = 2,
    MoveAverageLuminance = 3,
    EqualizeLuminanceAcrossFrames = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EqualizeFit {
    ScaleToFit = 0,
    ShiftToFit = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EqualizeLum {
    Luminance = 0,
    NtscLuminance = 1,
    MaxComponent = 2,
    MinComponent = 3,
    Average = 4,
    Red = 5,
    Green = 6,
    Blue = 7,
    FourthComponent = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EqualizeMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EqualizeFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EqualizeFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2EqualizeFmenu {
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
pub struct Cop2Equalize {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Equalize {
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

    /// Connects to input 0: "Image to Equalize"
    pub fn set_input_image_to_equalize<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to Equalize" and specifies the output index of the target node.
    pub fn set_input_image_to_equalize_from<N: crate::core::types::HoudiniNode>(
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
    pub fn with_black(mut self, val: f32) -> Self {
        self.params.insert(
            "black".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_black_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "black".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_white(mut self, val: f32) -> Self {
        self.params.insert(
            "white".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_white_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "white".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_average(mut self, val: f32) -> Self {
        self.params.insert(
            "average".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_average_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "average".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_before(mut self, val: i32) -> Self {
        self.params.insert(
            "before".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_before_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "before".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_after(mut self, val: i32) -> Self {
        self.params.insert(
            "after".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_after_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "after".to_string(),
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
    pub fn with_op(mut self, val: Cop2EqualizeOp) -> Self {
        self.params.insert(
            "op".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_op_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "op".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fit(mut self, val: Cop2EqualizeFit) -> Self {
        self.params.insert(
            "fit".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lum(mut self, val: Cop2EqualizeLum) -> Self {
        self.params.insert(
            "lum".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskinput(mut self, val: Cop2EqualizeMaskinput) -> Self {
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
    pub fn with_fscope(mut self, val: Cop2EqualizeFscope) -> Self {
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
    pub fn with_fdropfunc(mut self, val: Cop2EqualizeFdropfunc) -> Self {
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
    pub fn with_fmenu(mut self, val: Cop2EqualizeFmenu) -> Self {
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

impl crate::core::types::HoudiniNode for Cop2Equalize {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "equalize"
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
pub enum Cop2ErftablePlanes {
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
pub enum Cop2ErftableAddplanes {
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
pub enum Cop2ErftableAddplaneop {
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
pub enum Cop2ErftableDepth {
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
pub enum Cop2ErftableInterlace {
    None = 0,
    HalfResInterlaced = 1,
    BlackInterlaced = 2,
    LineDoubled = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ErftableIdominance {
    OddFirst = 0,
    EvenFirst = 1,
    OddOnly = 2,
    EvenOnly = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ErftablePreextend {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
    HoldNFrames = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ErftablePostextend {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
    HoldNFrames = 4,
}

#[derive(Debug, Clone)]
pub struct Cop2Erftable {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Erftable {
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
    pub fn with_max_sdev(mut self, val: f32) -> Self {
        self.params.insert(
            "max_sdev".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_max_sdev_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "max_sdev".to_string(),
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
    pub fn with_planes(mut self, val: Cop2ErftablePlanes) -> Self {
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
    pub fn with_addplanes(mut self, val: Cop2ErftableAddplanes) -> Self {
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
    pub fn with_addplaneop(mut self, val: Cop2ErftableAddplaneop) -> Self {
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
    pub fn with_depth(mut self, val: Cop2ErftableDepth) -> Self {
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
    pub fn with_interlace(mut self, val: Cop2ErftableInterlace) -> Self {
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
    pub fn with_idominance(mut self, val: Cop2ErftableIdominance) -> Self {
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
    pub fn with_preextend(mut self, val: Cop2ErftablePreextend) -> Self {
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
    pub fn with_postextend(mut self, val: Cop2ErftablePostextend) -> Self {
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

impl crate::core::types::HoudiniNode for Cop2Erftable {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "erftable"
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
pub enum Cop2ExpandOp {
    Expand = 0,
    Shrink = 1,
    FillHoles = 2,
    RemoveSpecks = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ExpandUnits {
    UvCoords = 0,
    Pixels = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ExpandDroptype {
    Linear = 0,
    Gaussian = 1,
    EaseIn = 2,
    EaseOut = 3,
    EaseInEaseOut = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ExpandLumfunc {
    Luminance = 0,
    NtscLuminance = 1,
    MaximumComponent = 2,
    MinimumComponent = 3,
    Average = 4,
    Red = 5,
    Green = 6,
    Blue = 7,
    Component4 = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ExpandMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ExpandFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ExpandFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ExpandFmenu {
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
pub struct Cop2Expand {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Expand {
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

    /// Connects to input 0: "Matte to Expand"
    pub fn set_input_matte_to_expand<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Matte to Expand" and specifies the output index of the target node.
    pub fn set_input_matte_to_expand_from<N: crate::core::types::HoudiniNode>(
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
    pub fn with_ysize(mut self, val: f32) -> Self {
        self.params.insert(
            "ysize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ysize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ysize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dropoff(mut self, val: f32) -> Self {
        self.params.insert(
            "dropoff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dropoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ydropoff(mut self, val: f32) -> Self {
        self.params.insert(
            "ydropoff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ydropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ydropoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_holethresh(mut self, val: f32) -> Self {
        self.params.insert(
            "holethresh".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_holethresh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "holethresh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_speckthresh(mut self, val: f32) -> Self {
        self.params.insert(
            "speckthresh".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_speckthresh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speckthresh".to_string(),
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
    pub fn with_op(mut self, val: Cop2ExpandOp) -> Self {
        self.params.insert(
            "op".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_op_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "op".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: Cop2ExpandUnits) -> Self {
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
    pub fn with_droptype(mut self, val: Cop2ExpandDroptype) -> Self {
        self.params.insert(
            "droptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_droptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "droptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lumfunc(mut self, val: Cop2ExpandLumfunc) -> Self {
        self.params.insert(
            "lumfunc".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lumfunc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lumfunc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskinput(mut self, val: Cop2ExpandMaskinput) -> Self {
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
    pub fn with_fscope(mut self, val: Cop2ExpandFscope) -> Self {
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
    pub fn with_fdropfunc(mut self, val: Cop2ExpandFdropfunc) -> Self {
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
    pub fn with_fmenu(mut self, val: Cop2ExpandFmenu) -> Self {
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
    pub fn with_useysize(mut self, val: bool) -> Self {
        self.params.insert(
            "useysize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useysize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useysize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useydrop(mut self, val: bool) -> Self {
        self.params.insert(
            "useydrop".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useydrop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useydrop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quality(mut self, val: bool) -> Self {
        self.params.insert(
            "quality".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_quality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uselum(mut self, val: bool) -> Self {
        self.params.insert(
            "uselum".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uselum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uselum".to_string(),
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

impl crate::core::types::HoudiniNode for Cop2Expand {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "expand"
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
pub enum Cop2ExtendExtendbefore {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ExtendExtendafter {
    BlackFrames = 0,
    Cycle = 1,
    Mirror = 2,
    Hold = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ExtendPreextend {
    NoChange = 0,
    BlackFrames = 1,
    Cycle = 2,
    Mirror = 3,
    Hold = 4,
    HoldNFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ExtendPostextend {
    NoChange = 0,
    BlackFrames = 1,
    Cycle = 2,
    Mirror = 3,
    Hold = 4,
    HoldNFrames = 5,
}

#[derive(Debug, Clone)]
pub struct Cop2Extend {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Extend {
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

    /// Connects to input 0: "Sequence to Extend"
    pub fn set_input_sequence_to_extend<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sequence to Extend" and specifies the output index of the target node.
    pub fn set_input_sequence_to_extend_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Sequence to Match"
    pub fn set_input_sequence_to_match<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Sequence to Match" and specifies the output index of the target node.
    pub fn set_input_sequence_to_match_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_numbefore(mut self, val: i32) -> Self {
        self.params.insert(
            "numbefore".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numbefore_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numbefore".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numafter(mut self, val: i32) -> Self {
        self.params.insert(
            "numafter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numafter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numafter".to_string(),
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

    // --- Menu parameters ---
    pub fn with_extendbefore(mut self, val: Cop2ExtendExtendbefore) -> Self {
        self.params.insert(
            "extendbefore".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_extendbefore_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extendbefore".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extendafter(mut self, val: Cop2ExtendExtendafter) -> Self {
        self.params.insert(
            "extendafter".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_extendafter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extendafter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preextend(mut self, val: Cop2ExtendPreextend) -> Self {
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
    pub fn with_postextend(mut self, val: Cop2ExtendPostextend) -> Self {
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
}

impl crate::core::types::HoudiniNode for Cop2Extend {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "extend"
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
pub enum Cop2ExtractExtract {
    Plane = 0,
    SingleChannel = 1,
}

#[derive(Debug, Clone)]
pub struct Cop2Extract {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Extract {
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

    /// Connects to input 0: "Source"
    pub fn set_input_source<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Source" and specifies the output index of the target node.
    pub fn set_input_source_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_extract(mut self, val: Cop2ExtractExtract) -> Self {
        self.params.insert(
            "extract".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_extract_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extract".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_channel(mut self, val: i32) -> Self {
        self.params.insert(
            "channel".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_channel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "channel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_planename(mut self, val: &str) -> Self {
        self.params.insert(
            "planename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_planename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rename(mut self, val: &str) -> Self {
        self.params.insert(
            "rename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for Cop2Extract {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "extract"
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
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait Cop2ExtractInnerExt {
    fn channelcopy1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn chromakey1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn convolve1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn delete1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn delete2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rename1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rename2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> Cop2ExtractInnerExt for crate::core::graph::InnerGraph<'a> {
    fn channelcopy1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("channelcopy1")
    }
    fn chromakey1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("chromakey1")
    }
    fn convolve1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("convolve1")
    }
    fn delete1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("delete1")
    }
    fn delete2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("delete2")
    }
    fn rename1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("rename1")
    }
    fn rename2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("rename2")
    }
    fn switch1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ExtrapolateboundariesBlendmethod {
    FillAverage = 0,
    FloodFill = 1,
    DiffuseFill = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ExtrapolateboundariesMaskinput {
    FirstInput = 0,
    MaskInput = 1,
    Off = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ExtrapolateboundariesFscope {
    AllFrames = 0,
    InsideRange = 1,
    OutsideRange = 2,
    EvenFrames = 3,
    OddFrames = 4,
    SpecificFrames = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ExtrapolateboundariesFdropfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cop2ExtrapolateboundariesFmenu {
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
pub struct Cop2Extrapolateboundaries {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl Cop2Extrapolateboundaries {
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

    /// Connects to input 0: "Image to Filter"
    pub fn set_input_image_to_filter<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Image to Filter" and specifies the output index of the target node.
    pub fn set_input_image_to_filter_from<N: crate::core::types::HoudiniNode>(
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
    pub fn with_maskthresh(mut self, val: f32) -> Self {
        self.params.insert(
            "maskthresh".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maskthresh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskthresh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visopacity(mut self, val: f32) -> Self {
        self.params.insert(
            "visopacity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_visopacity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visopacity".to_string(),
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

    // --- Float3 parameters ---
    pub fn with_viscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "viscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_viscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_edgepadding(mut self, val: i32) -> Self {
        self.params.insert(
            "edgepadding".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_edgepadding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "edgepadding".to_string(),
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
    pub fn with_blendmethod(mut self, val: Cop2ExtrapolateboundariesBlendmethod) -> Self {
        self.params.insert(
            "blendmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blendmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskinput(mut self, val: Cop2ExtrapolateboundariesMaskinput) -> Self {
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
    pub fn with_fscope(mut self, val: Cop2ExtrapolateboundariesFscope) -> Self {
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
    pub fn with_fdropfunc(mut self, val: Cop2ExtrapolateboundariesFdropfunc) -> Self {
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
    pub fn with_fmenu(mut self, val: Cop2ExtrapolateboundariesFmenu) -> Self {
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
    pub fn with_alphaasmask(mut self, val: bool) -> Self {
        self.params.insert(
            "alphaasmask".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alphaasmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alphaasmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_invertmask(mut self, val: bool) -> Self {
        self.params.insert(
            "invertmask".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_invertmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "invertmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visfill(mut self, val: bool) -> Self {
        self.params.insert(
            "visfill".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visfill_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visfill".to_string(),
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

impl crate::core::types::HoudiniNode for Cop2Extrapolateboundaries {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "extrapolateboundaries"
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
