#[derive(Debug, Clone)]
pub struct LopCache {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopCache {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "behavior".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "behavior".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trackprimexistence(mut self, val: bool) -> Self {
        self.params.insert(
            "trackprimexistence".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_trackprimexistence_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trackprimexistence".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopCache {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "cache"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait LopCacheOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopCacheOutputs for LopCache {}
impl LopCacheOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<LopCache> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCameraCreateprims {
    Edit = 0,
    Create = 1,
    /// Force Edit (Ignore Editable Flag)
    ForceEditIgnoreEditableFlag = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCameraXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCameraRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopCamera {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopCamera {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_twist(mut self, val: f32) -> Self {
        self.params.insert(
            "twist".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_twist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "twist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_focallength(mut self, val: f32) -> Self {
        self.params.insert(
            "focalLength".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_focallength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "focalLength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_focallengthconverted(mut self, val: f32) -> Self {
        self.params.insert(
            "focalLengthConverted".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_focallengthconverted_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "focalLengthConverted".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_horizontalaperture(mut self, val: f32) -> Self {
        self.params.insert(
            "horizontalAperture".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_horizontalaperture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "horizontalAperture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_horizontalapertureconverted(mut self, val: f32) -> Self {
        self.params.insert(
            "horizontalApertureConverted".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_horizontalapertureconverted_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "horizontalApertureConverted".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_verticalaperture(mut self, val: f32) -> Self {
        self.params.insert(
            "verticalAperture".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_verticalaperture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verticalAperture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_verticalapertureswitch(mut self, val: f32) -> Self {
        self.params.insert(
            "verticalApertureSwitch".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_verticalapertureswitch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verticalApertureSwitch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_verticalapertureconverted(mut self, val: f32) -> Self {
        self.params.insert(
            "verticalApertureConverted".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_verticalapertureconverted_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verticalApertureConverted".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_horizontalapertureoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "horizontalApertureOffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_horizontalapertureoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "horizontalApertureOffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_horizontalapertureoffsetconverted(mut self, val: f32) -> Self {
        self.params.insert(
            "horizontalApertureOffsetConverted".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_horizontalapertureoffsetconverted_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "horizontalApertureOffsetConverted".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_verticalapertureoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "verticalApertureOffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_verticalapertureoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verticalApertureOffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_verticalapertureoffsetconverted(mut self, val: f32) -> Self {
        self.params.insert(
            "verticalApertureOffsetConverted".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_verticalapertureoffsetconverted_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verticalApertureOffsetConverted".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_houdiniguidescale_s3a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__houdiniguidescale_s3a".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_houdiniguidescale_s3a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniguidescale_s3a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_shutteropen_0ta(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__shutteropen_0ta".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_shutteropen_0ta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__shutteropen_0ta".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_shutterclose_nva(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__shutterclose_nva".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_shutterclose_nva_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__shutterclose_nva".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_focusdistance(mut self, val: f32) -> Self {
        self.params.insert(
            "focusDistance".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_focusdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "focusDistance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fstop(mut self, val: f32) -> Self {
        self.params.insert(
            "fStop".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fstop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fStop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exposure(mut self, val: f32) -> Self {
        self.params.insert(
            "exposure".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_exposure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exposure".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clippingrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "clippingRange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_clippingrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clippingRange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aspectratio(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "aspectratio".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_aspectratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aspectratio".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shear".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shear".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatposition(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lookatposition".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lookatposition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatposition".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatprimpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lookatprimpos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lookatprimpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatprimpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatprimrot(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lookatprimrot".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lookatprimrot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatprimrot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upvec(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "upvec".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upvec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvec".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primcount(mut self, val: i32) -> Self {
        self.params.insert(
            "primcount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primcount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_createprims(mut self, val: LopCameraCreateprims) -> Self {
        self.params.insert(
            "createprims".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createprims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initforedit(mut self, val: i32) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_initforedit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xord(mut self, val: LopCameraXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rord(mut self, val: LopCameraRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "resMenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_resmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resMenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primtype(mut self, val: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind(mut self, val: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier(mut self, val: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_classancestor(mut self, val: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_classancestor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatprim(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upvecmethod(mut self, val: &str) -> Self {
        self.params.insert(
            "upvecmethod".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upvecmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvecmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_projection_control(mut self, val: &str) -> Self {
        self.params.insert(
            "projection_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_projection_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projection_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_projection(mut self, val: &str) -> Self {
        self.params.insert(
            "projection".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_projection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projection".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clippingrange_control(mut self, val: &str) -> Self {
        self.params.insert(
            "clippingRange_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clippingrange_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clippingRange_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_focallength_control(mut self, val: &str) -> Self {
        self.params.insert(
            "focalLength_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_focallength_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "focalLength_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aperture(mut self, val: &str) -> Self {
        self.params.insert(
            "aperture".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aperture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aperture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_horizontalaperture_control(mut self, val: &str) -> Self {
        self.params.insert(
            "horizontalAperture_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_horizontalaperture_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "horizontalAperture_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_verticalaperture_control(mut self, val: &str) -> Self {
        self.params.insert(
            "verticalAperture_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_verticalaperture_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verticalAperture_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_horizontalapertureoffset_control(mut self, val: &str) -> Self {
        self.params.insert(
            "horizontalApertureOffset_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_horizontalapertureoffset_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "horizontalApertureOffset_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_verticalapertureoffset_control(mut self, val: &str) -> Self {
        self.params.insert(
            "verticalApertureOffset_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_verticalapertureoffset_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verticalApertureOffset_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_houdiniguidescale_control_thb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__houdiniguidescale_control_thb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_houdiniguidescale_control_thb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniguidescale_control_thb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_houdiniinviewermenu_control_2kb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__houdiniinviewermenu_control_2kb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_houdiniinviewermenu_control_2kb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniinviewermenu_control_2kb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_houdinibackgroundimage_control_ypb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__houdinibackgroundimage_control_ypb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_houdinibackgroundimage_control_ypb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdinibackgroundimage_control_ypb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_houdinibackgroundimage_xcb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__houdinibackgroundimage_xcb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_houdinibackgroundimage_xcb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdinibackgroundimage_xcb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_houdiniforegroundimage_control_ypb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__houdiniforegroundimage_control_ypb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_houdiniforegroundimage_control_ypb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniforegroundimage_control_ypb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_houdiniforegroundimage_xcb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__houdiniforegroundimage_xcb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_houdiniforegroundimage_xcb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniforegroundimage_xcb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_shutteropen_control_16a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__shutteropen_control_16a".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_shutteropen_control_16a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__shutteropen_control_16a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_shutterclose_control_o8a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__shutterclose_control_o8a".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_shutterclose_control_o8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__shutterclose_control_o8a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_focusdistance_control(mut self, val: &str) -> Self {
        self.params.insert(
            "focusDistance_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_focusdistance_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "focusDistance_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fstop_control(mut self, val: &str) -> Self {
        self.params.insert(
            "fStop_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fstop_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fStop_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exposure_control(mut self, val: &str) -> Self {
        self.params.insert(
            "exposure_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exposure_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exposure_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatenable(mut self, val: bool) -> Self {
        self.params.insert(
            "lookatenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_lookatenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keepposition(mut self, val: bool) -> Self {
        self.params.insert(
            "keepposition".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepposition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepposition".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_houdiniinviewermenu_16a(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__houdiniinviewermenu_16a".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_houdiniinviewermenu_16a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniinviewermenu_16a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopCamera {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "camera"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait LopCameraOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopCameraOutputs for LopCamera {}
impl LopCameraOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<LopCamera> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCapsuleCreateprims {
    Edit = 0,
    Create = 1,
    /// Force Edit (Ignore Editable Flag)
    ForceEditIgnoreEditableFlag = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCapsuleXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCapsuleRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopCapsule {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopCapsule {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_height(mut self, val: f32) -> Self {
        self.params.insert(
            "height".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_height_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "height".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_radiusbottom(mut self, val: f32) -> Self {
        self.params.insert(
            "radiusBottom".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radiusbottom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radiusBottom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_radiustop(mut self, val: f32) -> Self {
        self.params.insert(
            "radiusTop".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radiustop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radiusTop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_ycb(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_ycb".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_ycb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_ycb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_p8a(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_p8a".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_p8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_p8a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shear".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shear".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primcount(mut self, val: i32) -> Self {
        self.params.insert(
            "primcount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primcount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_createprims(mut self, val: LopCapsuleCreateprims) -> Self {
        self.params.insert(
            "createprims".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createprims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initforedit(mut self, val: i32) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_initforedit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xord(mut self, val: LopCapsuleXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rord(mut self, val: LopCapsuleRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primtype(mut self, val: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind(mut self, val: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier(mut self, val: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_classancestor(mut self, val: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_classancestor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axis_control(mut self, val: &str) -> Self {
        self.params.insert(
            "axis_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axis_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axis_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axis(mut self, val: &str) -> Self {
        self.params.insert(
            "axis".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_height_control(mut self, val: &str) -> Self {
        self.params.insert(
            "height_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_height_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "height_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_radiusbottom_control(mut self, val: &str) -> Self {
        self.params.insert(
            "radiusBottom_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_radiusbottom_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radiusBottom_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_radiustop_control(mut self, val: &str) -> Self {
        self.params.insert(
            "radiusTop_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_radiustop_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radiusTop_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_control_qmb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_control_qmb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_control_qmb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_control_qmb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_control_zpb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_control_zpb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_control_zpb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_control_zpb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doublesided_control(mut self, val: &str) -> Self {
        self.params.insert(
            "doubleSided_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doublesided_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doubleSided_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computeextents(mut self, val: bool) -> Self {
        self.params.insert(
            "computeextents".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computeextents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computeextents".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doublesided(mut self, val: bool) -> Self {
        self.params.insert(
            "doubleSided".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doublesided_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doubleSided".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopCapsule {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "capsule"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait LopCapsuleOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopCapsuleOutputs for LopCapsule {}
impl LopCapsuleOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<LopCapsule> {}

#[derive(Debug, Clone)]
pub struct LopChopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopChopnet {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopChopnet {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "chopnet"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct LopCollection {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopCollection {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_defaultprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "defaultprimpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultprimpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collectionname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("collectionname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collectionname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("collectionname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("primpath{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("primpath{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_icon_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("icon{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_icon_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("icon{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expansionrule_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("expansionrule{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expansionrule_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("expansionrule{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_includepattern_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("includepattern{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_includepattern_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("includepattern{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_excludepattern_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("excludepattern{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_excludepattern_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("excludepattern{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_createprimitive(mut self, val: bool) -> Self {
        self.params.insert(
            "createprimitive".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createprimitive_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createprimitive".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addtolastmodified_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("addtolastmodified{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addtolastmodified_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("addtolastmodified{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_seticon_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("seticon{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_seticon_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("seticon{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_allowinstanceproxies_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("allowinstanceproxies{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowinstanceproxies_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("allowinstanceproxies{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ispathexpression_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("ispathexpression{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ispathexpression_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ispathexpression{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doexclusions_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("doexclusions{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doexclusions_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("doexclusions{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopCollection {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "collection"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait LopCollectionOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopCollectionOutputs for LopCollection {}
impl LopCollectionOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<LopCollection> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopComponentgeometrySourceinput {
    InternalSopNetwork = 0,
    ImportedFiles = 1,
    ExternalSopNetwork = 2,
    ReferencedFiles = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopComponentgeometryMissingframe {
    ReportError = 0,
    NoGeometry = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopComponentgeometryMaterialbindsubsets {
    DoNotCreateAutomaticPartitions = 0,
    CreateMaterialBindPartitions = 1,
}

#[derive(Debug, Clone)]
pub struct LopComponentgeometry {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopComponentgeometry {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn trigger_fixsopoutputs(mut self) -> Self {
        self.params.insert(
            "fixsopoutputs".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_reload(mut self) -> Self {
        self.params.insert(
            "reload".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_setupquickmaterials(mut self) -> Self {
        self.params.insert(
            "setupquickmaterials".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn with_setmissingwidths(mut self, val: f32) -> Self {
        self.params.insert(
            "setmissingwidths".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_setmissingwidths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setmissingwidths".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatafloatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("customdatafloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_customdatafloatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdatafloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_drawmodecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "drawmodecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_drawmodecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "drawmodecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceinput(mut self, val: LopComponentgeometrySourceinput) -> Self {
        self.params.insert(
            "sourceinput".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_sourceinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_materialbindsubsets(
        mut self,
        val: LopComponentgeometryMaterialbindsubsets,
    ) -> Self {
        self.params.insert(
            "materialbindsubsets".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_materialbindsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "materialbindsubsets".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataintvalue_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("customdataintvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_customdataintvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdataintvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_missingframe(mut self, val: LopComponentgeometryMissingframe) -> Self {
        self.params.insert(
            "missingframe".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_missingframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "missingframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_source(mut self, val: &str) -> Self {
        self.params.insert(
            "source".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_source_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceproxy(mut self, val: &str) -> Self {
        self.params.insert(
            "sourceproxy".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceproxy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceproxy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcesimproxy(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcesimproxy".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcesimproxy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcesimproxy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcesop(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcesop".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcesop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcesop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcesopproxy(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcesopproxy".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcesopproxy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcesopproxy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcesopsimproxy(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcesopsimproxy".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcesopsimproxy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcesopsimproxy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceusdref(mut self, val: &str) -> Self {
        self.params.insert(
            "sourceusdref".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceusdref_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceusdref".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceusdrefprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sourceusdrefprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceusdrefprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceusdrefprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceusdrefprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "sourceusdrefprimpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceusdrefprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceusdrefprimpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceproxyusdref(mut self, val: &str) -> Self {
        self.params.insert(
            "sourceproxyusdref".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceproxyusdref_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceproxyusdref".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceproxyusdrefprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sourceproxyusdrefprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceproxyusdrefprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceproxyusdrefprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceproxyusdrefprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "sourceproxyusdrefprimpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceproxyusdrefprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceproxyusdrefprimpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcesimproxyusdref(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcesimproxyusdref".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcesimproxyusdref_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcesimproxyusdref".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcesimproxyusdrefprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcesimproxyusdrefprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcesimproxyusdrefprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcesimproxyusdrefprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcesimproxyusdrefprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcesimproxyusdrefprimpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcesimproxyusdrefprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcesimproxyusdrefprimpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attribs(mut self, val: &str) -> Self {
        self.params.insert(
            "attribs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_indexattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "indexattribs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indexattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_partitionattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "partitionattribs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partitionattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_subsetgroups(mut self, val: &str) -> Self {
        self.params.insert(
            "subsetgroups".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsetgroups".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topologyhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "topologyhandling".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topologyhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topologyhandling".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_authortimesamples(mut self, val: &str) -> Self {
        self.params.insert(
            "authortimesamples".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_authortimesamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "authortimesamples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_packedhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "packedhandling".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_packedhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "packedhandling".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindmaterials(mut self, val: &str) -> Self {
        self.params.insert(
            "bindmaterials".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindmaterials_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindmaterials".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geovariantname(mut self, val: &str) -> Self {
        self.params.insert(
            "geovariantname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geovariantname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geovariantname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_icon(mut self, val: &str) -> Self {
        self.params.insert(
            "icon".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_icon_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "icon".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_drawmode(mut self, val: &str) -> Self {
        self.params.insert(
            "drawmode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_drawmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "drawmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customdataname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdataname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatatype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customdatatype{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatatype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdatatype{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatastrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customdatastrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatastrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdatastrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert(
            "prefixpartitionsubsets".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefixpartitionsubsets".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert(
            "translateuvtost".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "translateuvtost".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_polygonsassubd(mut self, val: bool) -> Self {
        self.params.insert(
            "polygonsassubd".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_polygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "polygonsassubd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_reversepolygons(mut self, val: bool) -> Self {
        self.params.insert(
            "reversepolygons".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reversepolygons_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reversepolygons".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enable_setmissingwidths(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_setmissingwidths".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_setmissingwidths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_setmissingwidths".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extentshint(mut self, val: bool) -> Self {
        self.params.insert(
            "extentshint".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_extentshint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extentshint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_heightfieldconvert(mut self, val: bool) -> Self {
        self.params.insert(
            "heightfieldconvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_heightfieldconvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "heightfieldconvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_quickmaterialinside(mut self, val: bool) -> Self {
        self.params.insert(
            "quickmaterialinside".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_quickmaterialinside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quickmaterialinside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_instancepayloadgeo(mut self, val: bool) -> Self {
        self.params.insert(
            "instancepayloadgeo".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_instancepayloadgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instancepayloadgeo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usesceneimport(mut self, val: bool) -> Self {
        self.params.insert(
            "usesceneimport".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesceneimport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesceneimport".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_seticon(mut self, val: bool) -> Self {
        self.params.insert(
            "seticon".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_seticon_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seticon".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enforcemodelkind(mut self, val: bool) -> Self {
        self.params.insert(
            "enforcemodelkind".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enforcemodelkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enforcemodelkind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dogeommodelapi(mut self, val: bool) -> Self {
        self.params.insert(
            "dogeommodelapi".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dogeommodelapi_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dogeommodelapi".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataboolvalue_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("customdataboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_customdataboolvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdataboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopComponentgeometry {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "componentgeometry"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
    fn get_dive_target(&self) -> Option<&'static str> {
        Some("sopnet/geo")
    }
}

pub trait LopComponentgeometryOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopComponentgeometryOutputs for LopComponentgeometry {}
impl LopComponentgeometryOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<LopComponentgeometry>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait LopComponentgeometryInnerExt {
    fn default(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn proxy(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn simproxy(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> LopComponentgeometryInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, LopComponentgeometry>
{
    fn default(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sopnet/geo/default")
    }
    fn proxy(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sopnet/geo/proxy")
    }
    fn simproxy(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sopnet/geo/simproxy")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopComponentgeometryvariantsVariantsource {
    Inputs = 0,
    Number = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopComponentgeometryvariantsVariantnamesrc {
    ComponentGeometryInputs = 0,
    Manual = 1,
}

#[derive(Debug, Clone)]
pub struct LopComponentgeometryvariants {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopComponentgeometryvariants {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }
    pub fn set_input_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(1, (out.node_id, out.pin));
        self
    }
    pub fn set_input_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(2, (out.node_id, out.pin));
        self
    }
    pub fn set_input_4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(3, (out.node_id, out.pin));
        self
    }
    pub fn set_input_5_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(4, (out.node_id, out.pin));
        self
    }
    pub fn set_input_6_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(5, (out.node_id, out.pin));
        self
    }
    pub fn set_input_7_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(6, (out.node_id, out.pin));
        self
    }
    pub fn set_input_8_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(7, (out.node_id, out.pin));
        self
    }
    pub fn set_input_9_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(8, (out.node_id, out.pin));
        self
    }
    pub fn set_input_10_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(9, (out.node_id, out.pin));
        self
    }
    pub fn set_input_11_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(10, (out.node_id, out.pin));
        self
    }
    pub fn set_input_12_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(11, (out.node_id, out.pin));
        self
    }
    pub fn set_input_13_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(12, (out.node_id, out.pin));
        self
    }
    pub fn set_input_14_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(13, (out.node_id, out.pin));
        self
    }
    pub fn set_input_15_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(14, (out.node_id, out.pin));
        self
    }
    pub fn set_input_16_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(15, (out.node_id, out.pin));
        self
    }
    pub fn set_input_17_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(16, (out.node_id, out.pin));
        self
    }
    pub fn set_input_18_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(17, (out.node_id, out.pin));
        self
    }
    pub fn set_input_19_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(18, (out.node_id, out.pin));
        self
    }
    pub fn set_input_20_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(19, (out.node_id, out.pin));
        self
    }
    pub fn set_input_21_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(20, (out.node_id, out.pin));
        self
    }
    pub fn set_input_22_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(21, (out.node_id, out.pin));
        self
    }
    pub fn set_input_23_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(22, (out.node_id, out.pin));
        self
    }
    pub fn set_input_24_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(23, (out.node_id, out.pin));
        self
    }
    pub fn set_input_25_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(24, (out.node_id, out.pin));
        self
    }
    pub fn set_input_26_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(25, (out.node_id, out.pin));
        self
    }
    pub fn set_input_27_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(26, (out.node_id, out.pin));
        self
    }
    pub fn set_input_28_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(27, (out.node_id, out.pin));
        self
    }
    pub fn set_input_29_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(28, (out.node_id, out.pin));
        self
    }
    pub fn set_input_30_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(29, (out.node_id, out.pin));
        self
    }
    pub fn set_input_31_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(30, (out.node_id, out.pin));
        self
    }
    pub fn set_input_32_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(31, (out.node_id, out.pin));
        self
    }
    pub fn set_input_33_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(32, (out.node_id, out.pin));
        self
    }
    pub fn set_input_34_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(33, (out.node_id, out.pin));
        self
    }
    pub fn set_input_35_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(34, (out.node_id, out.pin));
        self
    }
    pub fn set_input_36_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(35, (out.node_id, out.pin));
        self
    }
    pub fn set_input_37_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(36, (out.node_id, out.pin));
        self
    }
    pub fn set_input_38_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(37, (out.node_id, out.pin));
        self
    }
    pub fn set_input_39_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(38, (out.node_id, out.pin));
        self
    }
    pub fn set_input_40_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(39, (out.node_id, out.pin));
        self
    }
    pub fn set_input_41_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(40, (out.node_id, out.pin));
        self
    }
    pub fn set_input_42_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(41, (out.node_id, out.pin));
        self
    }
    pub fn set_input_43_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(42, (out.node_id, out.pin));
        self
    }
    pub fn set_input_44_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(43, (out.node_id, out.pin));
        self
    }
    pub fn set_input_45_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(44, (out.node_id, out.pin));
        self
    }
    pub fn set_input_46_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(45, (out.node_id, out.pin));
        self
    }
    pub fn set_input_47_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(46, (out.node_id, out.pin));
        self
    }
    pub fn set_input_48_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(47, (out.node_id, out.pin));
        self
    }
    pub fn set_input_49_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(48, (out.node_id, out.pin));
        self
    }
    pub fn set_input_50_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(49, (out.node_id, out.pin));
        self
    }
    pub fn set_input_51_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(50, (out.node_id, out.pin));
        self
    }
    pub fn set_input_52_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(51, (out.node_id, out.pin));
        self
    }
    pub fn set_input_53_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(52, (out.node_id, out.pin));
        self
    }
    pub fn set_input_54_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(53, (out.node_id, out.pin));
        self
    }
    pub fn set_input_55_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(54, (out.node_id, out.pin));
        self
    }
    pub fn set_input_56_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(55, (out.node_id, out.pin));
        self
    }
    pub fn set_input_57_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(56, (out.node_id, out.pin));
        self
    }
    pub fn set_input_58_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(57, (out.node_id, out.pin));
        self
    }
    pub fn set_input_59_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(58, (out.node_id, out.pin));
        self
    }
    pub fn set_input_60_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(59, (out.node_id, out.pin));
        self
    }
    pub fn set_input_61_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(60, (out.node_id, out.pin));
        self
    }
    pub fn set_input_62_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(61, (out.node_id, out.pin));
        self
    }
    pub fn set_input_63_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(62, (out.node_id, out.pin));
        self
    }
    pub fn set_input_64_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(63, (out.node_id, out.pin));
        self
    }
    pub fn set_input_65_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(64, (out.node_id, out.pin));
        self
    }
    pub fn set_input_66_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(65, (out.node_id, out.pin));
        self
    }
    pub fn set_input_67_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(66, (out.node_id, out.pin));
        self
    }
    pub fn set_input_68_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(67, (out.node_id, out.pin));
        self
    }
    pub fn set_input_69_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(68, (out.node_id, out.pin));
        self
    }
    pub fn set_input_70_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(69, (out.node_id, out.pin));
        self
    }
    pub fn set_input_71_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(70, (out.node_id, out.pin));
        self
    }
    pub fn set_input_72_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(71, (out.node_id, out.pin));
        self
    }
    pub fn set_input_73_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(72, (out.node_id, out.pin));
        self
    }
    pub fn set_input_74_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(73, (out.node_id, out.pin));
        self
    }
    pub fn set_input_75_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(74, (out.node_id, out.pin));
        self
    }
    pub fn set_input_76_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(75, (out.node_id, out.pin));
        self
    }
    pub fn set_input_77_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(76, (out.node_id, out.pin));
        self
    }
    pub fn set_input_78_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(77, (out.node_id, out.pin));
        self
    }
    pub fn set_input_79_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(78, (out.node_id, out.pin));
        self
    }
    pub fn set_input_80_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(79, (out.node_id, out.pin));
        self
    }
    pub fn set_input_81_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(80, (out.node_id, out.pin));
        self
    }
    pub fn set_input_82_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(81, (out.node_id, out.pin));
        self
    }
    pub fn set_input_83_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(82, (out.node_id, out.pin));
        self
    }
    pub fn set_input_84_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(83, (out.node_id, out.pin));
        self
    }
    pub fn set_input_85_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(84, (out.node_id, out.pin));
        self
    }
    pub fn set_input_86_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(85, (out.node_id, out.pin));
        self
    }
    pub fn set_input_87_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(86, (out.node_id, out.pin));
        self
    }
    pub fn set_input_88_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(87, (out.node_id, out.pin));
        self
    }
    pub fn set_input_89_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(88, (out.node_id, out.pin));
        self
    }
    pub fn set_input_90_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(89, (out.node_id, out.pin));
        self
    }
    pub fn set_input_91_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(90, (out.node_id, out.pin));
        self
    }
    pub fn set_input_92_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(91, (out.node_id, out.pin));
        self
    }
    pub fn set_input_93_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(92, (out.node_id, out.pin));
        self
    }
    pub fn set_input_94_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(93, (out.node_id, out.pin));
        self
    }
    pub fn set_input_95_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(94, (out.node_id, out.pin));
        self
    }
    pub fn set_input_96_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(95, (out.node_id, out.pin));
        self
    }
    pub fn set_input_97_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(96, (out.node_id, out.pin));
        self
    }
    pub fn set_input_98_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(97, (out.node_id, out.pin));
        self
    }
    pub fn set_input_99_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(98, (out.node_id, out.pin));
        self
    }

    pub fn with_variantsource(mut self, val: LopComponentgeometryvariantsVariantsource) -> Self {
        self.params.insert(
            "variantsource".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_variantsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantsource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantcount(mut self, val: i32) -> Self {
        self.params.insert(
            "variantcount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_variantcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantcount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantnamesrc(mut self, val: LopComponentgeometryvariantsVariantnamesrc) -> Self {
        self.params.insert(
            "variantnamesrc".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_variantnamesrc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantnamesrc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantset(mut self, val: &str) -> Self {
        self.params.insert(
            "variantset".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantprefix(mut self, val: &str) -> Self {
        self.params.insert(
            "variantprefix".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantprefix".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantname(mut self, val: &str) -> Self {
        self.params.insert(
            "variantname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantcountname(mut self, val: &str) -> Self {
        self.params.insert(
            "variantcountname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantcountname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantcountname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantname1(mut self, val: &str) -> Self {
        self.params.insert(
            "variantname1".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantname1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantname1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setcurrentselection(mut self, val: bool) -> Self {
        self.params.insert(
            "setcurrentselection".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setcurrentselection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setcurrentselection".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopComponentgeometryvariants {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "componentgeometryvariants"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait LopComponentgeometryvariantsOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopComponentgeometryvariantsOutputs for LopComponentgeometryvariants {}
impl LopComponentgeometryvariantsOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<LopComponentgeometryvariants>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopComponentmaterialSecondinputbehavior {
    Unchanged = 0,
    Copy = 1,
    CopyExclusive = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopComponentmaterialBindstrength {
    Default = 0,
    StrongerThanDescendants = 1,
    WeakerThanDescendants = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopComponentmaterialBindmethod {
    Direct = 0,
    CollectionBased = 1,
}

#[derive(Debug, Clone)]
pub struct LopComponentmaterial {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopComponentmaterial {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }
    pub fn set_materials_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(1, (out.node_id, out.pin));
        self
    }

    pub fn trigger_addmateriallibrary(mut self) -> Self {
        self.params.insert(
            "addmateriallibrary".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn with_secondinputbehavior(
        mut self,
        val: LopComponentmaterialSecondinputbehavior,
    ) -> Self {
        self.params.insert(
            "secondinputbehavior".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_secondinputbehavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "secondinputbehavior".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_quickmaterials(mut self, val: i32) -> Self {
        self.params.insert(
            "quickmaterials".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_quickmaterials_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quickmaterials".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_quickmat_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("quickmat{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_quickmat_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("quickmat{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindstrength_inst(
        mut self,
        index1: usize,
        val: LopComponentmaterialBindstrength,
    ) -> Self {
        self.params.insert(
            format!("bindstrength{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindstrength_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindstrength{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindmethod_inst(
        mut self,
        index1: usize,
        val: LopComponentmaterialBindmethod,
    ) -> Self {
        self.params.insert(
            format!("bindmethod{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindmethod_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindmethod{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantset(mut self, val: &str) -> Self {
        self.params.insert(
            "variantset".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantname(mut self, val: &str) -> Self {
        self.params.insert(
            "variantname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_root(mut self, val: &str) -> Self {
        self.params.insert(
            "root".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_root_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "root".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_quickmatlocation(mut self, val: &str) -> Self {
        self.params.insert(
            "quickmatlocation".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_quickmatlocation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quickmatlocation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("primpattern{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("primpattern{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_matspecpath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("matspecpath{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_matspecpath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("matspecpath{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindpath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("bindpath{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindpath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindpath{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("bindname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_autoroot(mut self, val: bool) -> Self {
        self.params.insert(
            "autoroot".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_autoroot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "autoroot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_simpleparms(mut self, val: bool) -> Self {
        self.params.insert(
            "simpleparms".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_simpleparms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "simpleparms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_quickmatuseblankmaterials(mut self, val: bool) -> Self {
        self.params.insert(
            "quickmatuseblankmaterials".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_quickmatuseblankmaterials_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quickmatuseblankmaterials".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindcollectionexpand_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("bindcollectionexpand{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bindcollectionexpand_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bindcollectionexpand{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopComponentmaterial {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "componentmaterial"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
    fn get_dive_target(&self) -> Option<&'static str> {
        Some("edit")
    }
}

pub trait LopComponentmaterialOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopComponentmaterialOutputs for LopComponentmaterial {}
impl LopComponentmaterialOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<LopComponentmaterial>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait LopComponentmaterialInnerExt {
    fn output0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> LopComponentmaterialInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, LopComponentmaterial>
{
    fn output0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("edit/output0")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopComponentoutputMode {
    UpstreamComponentGeometry = 0,
    InputPrimitives = 1,
    SceneImport = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopComponentoutputThumbnailmode {
    ExistingImage = 0,
    HoudiniGl = 1,
    Render = 2,
    Viewport = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopComponentoutputThumbnailscenesource {
    Automatic = 0,
    SecondInput = 1,
}

#[derive(Debug, Clone)]
pub struct LopComponentoutput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopComponentoutput {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }
    pub fn set_thumbnail_scene_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(1, (out.node_id, out.pin));
        self
    }

    pub fn trigger_execute(mut self) -> Self {
        self.params.insert(
            "execute".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_executebackground(mut self) -> Self {
        self.params.insert(
            "executebackground".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_reload(mut self) -> Self {
        self.params.insert(
            "reload".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_addtogallery(mut self) -> Self {
        self.params.insert(
            "addtogallery".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_executegl(mut self) -> Self {
        self.params.insert(
            "executegl".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_executerender(mut self) -> Self {
        self.params.insert(
            "executerender".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_executefile(mut self) -> Self {
        self.params.insert(
            "executefile".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_executeviewport(mut self) -> Self {
        self.params.insert(
            "executeviewport".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_executesavethumbnail(mut self) -> Self {
        self.params.insert(
            "executesavethumbnail".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_thumbnailquicksetup(mut self) -> Self {
        self.params.insert(
            "thumbnailquicksetup".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn with_customdatafloatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("customdatafloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_customdatafloatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdatafloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_metersperunit(mut self, val: f32) -> Self {
        self.params.insert(
            "metersperunit".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_metersperunit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "metersperunit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optionfloatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("optionfloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_optionfloatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionfloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spin(mut self, val: f32) -> Self {
        self.params.insert(
            "spin".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spin".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pitch(mut self, val: f32) -> Self {
        self.params.insert(
            "pitch".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pitch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pitch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dist(mut self, val: f32) -> Self {
        self.params.insert(
            "dist".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mode(mut self, val: LopComponentoutputMode) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataintvalue_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("customdataintvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_customdataintvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdataintvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thumbnailmode(mut self, val: LopComponentoutputThumbnailmode) -> Self {
        self.params.insert(
            "thumbnailmode".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_thumbnailmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thumbnailmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thumbnailscenesource(
        mut self,
        val: LopComponentoutputThumbnailscenesource,
    ) -> Self {
        self.params.insert(
            "thumbnailscenesource".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_thumbnailscenesource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thumbnailscenesource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_res(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "res".to_string(),
            houdini_ramen_core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_res_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rootprim(mut self, val: &str) -> Self {
        self.params.insert(
            "rootprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rootprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rootprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kind(mut self, val: &str) -> Self {
        self.params.insert(
            "kind".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_name(mut self, val: &str) -> Self {
        self.params.insert(
            "__name".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "__name".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantdefaultgeo(mut self, val: &str) -> Self {
        self.params.insert(
            "variantdefaultgeo".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantdefaultgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantdefaultgeo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantdefaultmtl(mut self, val: &str) -> Self {
        self.params.insert(
            "variantdefaultmtl".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantdefaultmtl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantdefaultmtl".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetidentifier(mut self, val: &str) -> Self {
        self.params.insert(
            "assetidentifier".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetidentifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "assetidentifier".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetname(mut self, val: &str) -> Self {
        self.params.insert(
            "assetname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "assetname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetversion(mut self, val: &str) -> Self {
        self.params.insert(
            "assetversion".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetversion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "assetversion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetdependencies(mut self, val: &str) -> Self {
        self.params.insert(
            "assetdependencies".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetdependencies_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "assetdependencies".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_icon(mut self, val: &str) -> Self {
        self.params.insert(
            "icon".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_icon_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "icon".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customdataname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdataname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatatype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customdatatype{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatatype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdatatype{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatastrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customdatastrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatastrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdatastrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_name_1(mut self, val: &str) -> Self {
        self.params.insert(
            "name".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_name_1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filename(mut self, val: &str) -> Self {
        self.params.insert(
            "filename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lopoutput(mut self, val: &str) -> Self {
        self.params.insert(
            "lopoutput".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lopoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lopoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_handlemissingfiles(mut self, val: &str) -> Self {
        self.params.insert(
            "handlemissingfiles".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_handlemissingfiles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handlemissingfiles".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_payloadlayer(mut self, val: &str) -> Self {
        self.params.insert(
            "payloadlayer".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_payloadlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "payloadlayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geolayer(mut self, val: &str) -> Self {
        self.params.insert(
            "geolayer".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geolayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geolayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mtllayer(mut self, val: &str) -> Self {
        self.params.insert(
            "mtllayer".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mtllayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mtllayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extralayer(mut self, val: &str) -> Self {
        self.params.insert(
            "extralayer".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extralayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extralayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_manualpayloadlayer(mut self, val: &str) -> Self {
        self.params.insert(
            "manualpayloadlayer".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_manualpayloadlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "manualpayloadlayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localizesubdir(mut self, val: &str) -> Self {
        self.params.insert(
            "localizesubdir".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localizesubdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localizesubdir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_recipeoutput(mut self, val: &str) -> Self {
        self.params.insert(
            "recipeoutput".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_recipeoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "recipeoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_recipename(mut self, val: &str) -> Self {
        self.params.insert(
            "recipename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_recipename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "recipename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upaxis(mut self, val: &str) -> Self {
        self.params.insert(
            "upaxis".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantlayersset(mut self, val: &str) -> Self {
        self.params.insert(
            "variantlayersset".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantlayersset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantlayersset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantlayersdir(mut self, val: &str) -> Self {
        self.params.insert(
            "variantlayersdir".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantlayersdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantlayersdir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_savestyle(mut self, val: &str) -> Self {
        self.params.insert(
            "savestyle".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_savestyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savestyle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optionname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optionname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optionname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optiontype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optiontype{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optiontype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optiontype{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optionstrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optionstrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optionstrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionstrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_renderer(mut self, val: &str) -> Self {
        self.params.insert(
            "renderer".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_renderer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thumbnailfile(mut self, val: &str) -> Self {
        self.params.insert(
            "thumbnailfile".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thumbnailfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thumbnailfile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thumbnailinputcamera(mut self, val: &str) -> Self {
        self.params.insert(
            "thumbnailinputcamera".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thumbnailinputcamera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thumbnailinputcamera".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setdefaultvariants(mut self, val: bool) -> Self {
        self.params.insert(
            "setdefaultvariants".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setdefaultvariants_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setdefaultvariants".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doclassinherit(mut self, val: bool) -> Self {
        self.params.insert(
            "doclassinherit".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doclassinherit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doclassinherit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setvariantsinside(mut self, val: bool) -> Self {
        self.params.insert(
            "setvariantsinside".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setvariantsinside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setvariantsinside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_payloadinputprims(mut self, val: bool) -> Self {
        self.params.insert(
            "payloadinputprims".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_payloadinputprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "payloadinputprims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dogeoinstance(mut self, val: bool) -> Self {
        self.params.insert(
            "dogeoinstance".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dogeoinstance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dogeoinstance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setinstanceable(mut self, val: bool) -> Self {
        self.params.insert(
            "setinstanceable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setinstanceable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setinstanceable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setassetidentifier(mut self, val: bool) -> Self {
        self.params.insert(
            "setassetidentifier".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setassetidentifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setassetidentifier".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setassetname(mut self, val: bool) -> Self {
        self.params.insert(
            "setassetname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setassetname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setassetname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setassetversion(mut self, val: bool) -> Self {
        self.params.insert(
            "setassetversion".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setassetversion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setassetversion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setassetdependencies(mut self, val: bool) -> Self {
        self.params.insert(
            "setassetdependencies".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setassetdependencies_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setassetdependencies".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_seticon(mut self, val: bool) -> Self {
        self.params.insert(
            "seticon".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_seticon_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seticon".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataboolvalue_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("customdataboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_customdataboolvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdataboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loadfromdisk(mut self, val: bool) -> Self {
        self.params.insert(
            "loadfromdisk".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loadfromdisk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loadfromdisk".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantlayers(mut self, val: bool) -> Self {
        self.params.insert(
            "variantlayers".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_variantlayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantlayers".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localize(mut self, val: bool) -> Self {
        self.params.insert(
            "localize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_localize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_saverecipe(mut self, val: bool) -> Self {
        self.params.insert(
            "saverecipe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_saverecipe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "saverecipe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_striplayerbreaks(mut self, val: bool) -> Self {
        self.params.insert(
            "striplayerbreaks".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_striplayerbreaks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "striplayerbreaks".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fileperframe(mut self, val: bool) -> Self {
        self.params.insert(
            "fileperframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fileperframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fileperframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_requiredefaultprim(mut self, val: bool) -> Self {
        self.params.insert(
            "requiredefaultprim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_requiredefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requiredefaultprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_savetimeinfo(mut self, val: bool) -> Self {
        self.params.insert(
            "savetimeinfo".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savetimeinfo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savetimeinfo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clearhoudinicustomdata(mut self, val: bool) -> Self {
        self.params.insert(
            "clearhoudinicustomdata".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clearhoudinicustomdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clearhoudinicustomdata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ensuremetricsset(mut self, val: bool) -> Self {
        self.params.insert(
            "ensuremetricsset".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ensuremetricsset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ensuremetricsset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_optionenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("optionenable{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_optionenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionenable{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tprerender(mut self, val: bool) -> Self {
        self.params.insert(
            "tprerender".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tprerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tpreframe(mut self, val: bool) -> Self {
        self.params.insert(
            "tpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tpostframe(mut self, val: bool) -> Self {
        self.params.insert(
            "tpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tpostrender(mut self, val: bool) -> Self {
        self.params.insert(
            "tpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initsim(mut self, val: bool) -> Self {
        self.params.insert(
            "initsim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_initsim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initsim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_alfprogress(mut self, val: bool) -> Self {
        self.params.insert(
            "alfprogress".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alfprogress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alfprogress".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_reportnetwork(mut self, val: bool) -> Self {
        self.params.insert(
            "reportnetwork".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reportnetwork_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reportnetwork".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_autothumbnail(mut self, val: bool) -> Self {
        self.params.insert(
            "autothumbnail".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_autothumbnail_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "autothumbnail".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thumbnailexportlayer(mut self, val: bool) -> Self {
        self.params.insert(
            "thumbnailexportlayer".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_thumbnailexportlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thumbnailexportlayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_viewthumbnailcam(mut self, val: bool) -> Self {
        self.params.insert(
            "viewthumbnailcam".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viewthumbnailcam_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viewthumbnailcam".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopComponentoutput {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "componentoutput"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
    fn get_dive_target(&self) -> Option<&'static str> {
        Some("extras")
    }
}

pub trait LopComponentoutputOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopComponentoutputOutputs for LopComponentoutput {}
impl LopComponentoutputOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<LopComponentoutput>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait LopComponentoutputInnerExt {
    fn output0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> LopComponentoutputInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, LopComponentoutput>
{
    fn output0(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("extras/output0")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopConeCreateprims {
    Edit = 0,
    Create = 1,
    /// Force Edit (Ignore Editable Flag)
    ForceEditIgnoreEditableFlag = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopConeXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopConeRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopCone {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopCone {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_height(mut self, val: f32) -> Self {
        self.params.insert(
            "height".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_height_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "height".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "radius".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radius".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_ycb(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_ycb".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_ycb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_ycb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_p8a(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_p8a".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_p8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_p8a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shear".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shear".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primcount(mut self, val: i32) -> Self {
        self.params.insert(
            "primcount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primcount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_createprims(mut self, val: LopConeCreateprims) -> Self {
        self.params.insert(
            "createprims".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createprims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initforedit(mut self, val: i32) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_initforedit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xord(mut self, val: LopConeXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rord(mut self, val: LopConeRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primtype(mut self, val: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind(mut self, val: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier(mut self, val: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_classancestor(mut self, val: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_classancestor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axis_control(mut self, val: &str) -> Self {
        self.params.insert(
            "axis_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axis_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axis_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axis(mut self, val: &str) -> Self {
        self.params.insert(
            "axis".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_height_control(mut self, val: &str) -> Self {
        self.params.insert(
            "height_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_height_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "height_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_radius_control(mut self, val: &str) -> Self {
        self.params.insert(
            "radius_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_radius_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radius_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_control_qmb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_control_qmb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_control_qmb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_control_qmb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_control_zpb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_control_zpb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_control_zpb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_control_zpb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doublesided_control(mut self, val: &str) -> Self {
        self.params.insert(
            "doubleSided_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doublesided_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doubleSided_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computeextents(mut self, val: bool) -> Self {
        self.params.insert(
            "computeextents".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computeextents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computeextents".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doublesided(mut self, val: bool) -> Self {
        self.params.insert(
            "doubleSided".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doublesided_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doubleSided".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopCone {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "cone"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait LopConeOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopConeOutputs for LopCone {}
impl LopConeOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<LopCone> {}

#[derive(Debug, Clone)]
pub struct LopConfigurelayer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopConfigurelayer {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_starttime(mut self, val: f32) -> Self {
        self.params.insert(
            "starttime".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_starttime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "starttime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_endtime(mut self, val: f32) -> Self {
        self.params.insert(
            "endtime".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_endtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "endtime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_timepersecond(mut self, val: f32) -> Self {
        self.params.insert(
            "timepersecond".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timepersecond_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timepersecond".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_framespersecond(mut self, val: f32) -> Self {
        self.params.insert(
            "framespersecond".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_framespersecond_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "framespersecond".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_metersperunit(mut self, val: f32) -> Self {
        self.params.insert(
            "metersperunit".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_metersperunit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "metersperunit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatafloatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("customdatafloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_customdatafloatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdatafloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablesfloatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("stagevariablesfloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stagevariablesfloatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stagevariablesfloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataintvalue_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("customdataintvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_customdataintvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdataintvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablesintvalue_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("stagevariablesintvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stagevariablesintvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stagevariablesintvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_savepath(mut self, val: &str) -> Self {
        self.params.insert(
            "savepath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_savepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savepath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultprim(mut self, val: &str) -> Self {
        self.params.insert(
            "defaultprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_comment(mut self, val: &str) -> Self {
        self.params.insert(
            "comment".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_comment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comment".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upaxis(mut self, val: &str) -> Self {
        self.params.insert(
            "upaxis".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_colorconfig(mut self, val: &str) -> Self {
        self.params.insert(
            "colorconfig".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_colorconfig_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorconfig".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_colormanagementsystem(mut self, val: &str) -> Self {
        self.params.insert(
            "colormanagementsystem".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_colormanagementsystem_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colormanagementsystem".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rendersettings(mut self, val: &str) -> Self {
        self.params.insert(
            "rendersettings".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rendersettings_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rendersettings".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customdataname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdataname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatatype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customdatatype{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatatype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdatatype{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatastrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customdatastrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatastrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdatastrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablesname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("stagevariablesname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablesname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stagevariablesname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablestype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("stagevariablestype{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablestype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stagevariablestype{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablesstrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("stagevariablesstrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablesstrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stagevariablesstrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flattenop(mut self, val: &str) -> Self {
        self.params.insert(
            "flattenop".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flattenop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flattenop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setsavepath(mut self, val: bool) -> Self {
        self.params.insert(
            "setsavepath".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setsavepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setsavepath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setdefaultprim(mut self, val: bool) -> Self {
        self.params.insert(
            "setdefaultprim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setdefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setdefaultprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setcomment(mut self, val: bool) -> Self {
        self.params.insert(
            "setcomment".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setcomment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setcomment".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setstarttime(mut self, val: bool) -> Self {
        self.params.insert(
            "setstarttime".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setstarttime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setstarttime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setendtime(mut self, val: bool) -> Self {
        self.params.insert(
            "setendtime".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setendtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setendtime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_settimepersecond(mut self, val: bool) -> Self {
        self.params.insert(
            "settimepersecond".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_settimepersecond_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "settimepersecond".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setframespersecond(mut self, val: bool) -> Self {
        self.params.insert(
            "setframespersecond".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setframespersecond_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setframespersecond".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setupaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "setupaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setupaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setupaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setmetersperunit(mut self, val: bool) -> Self {
        self.params.insert(
            "setmetersperunit".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setmetersperunit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setmetersperunit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setcolorconfig(mut self, val: bool) -> Self {
        self.params.insert(
            "setcolorconfig".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setcolorconfig_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setcolorconfig".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setcolormanagementsystem(mut self, val: bool) -> Self {
        self.params.insert(
            "setcolormanagementsystem".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setcolormanagementsystem_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setcolormanagementsystem".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setrendersettings(mut self, val: bool) -> Self {
        self.params.insert(
            "setrendersettings".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setrendersettings_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setrendersettings".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataboolvalue_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("customdataboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_customdataboolvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdataboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablesboolvalue_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("stagevariablesboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stagevariablesboolvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stagevariablesboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setstagemetadata(mut self, val: bool) -> Self {
        self.params.insert(
            "setstagemetadata".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setstagemetadata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setstagemetadata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_startnewlayer(mut self, val: bool) -> Self {
        self.params.insert(
            "startnewlayer".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_startnewlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "startnewlayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setinputlayerexplicit(mut self, val: bool) -> Self {
        self.params.insert(
            "setinputlayerexplicit".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setinputlayerexplicit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setinputlayerexplicit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopConfigurelayer {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "configurelayer"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait LopConfigurelayerOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopConfigurelayerOutputs for LopConfigurelayer {}
impl LopConfigurelayerOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<LopConfigurelayer>
{
}

#[derive(Debug, Clone)]
pub struct LopConfigureprimitive {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopConfigureprimitive {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_assetinfofloatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("assetinfofloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_assetinfofloatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("assetinfofloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatafloatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("customdatafloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_customdatafloatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdatafloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_drawmodecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "drawmodecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_drawmodecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "drawmodecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfointvalue_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("assetinfointvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_assetinfointvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("assetinfointvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataintvalue_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("customdataintvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_customdataintvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdataintvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_purpose(mut self, val: &str) -> Self {
        self.params.insert(
            "purpose".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_purpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "purpose".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proxy(mut self, val: &str) -> Self {
        self.params.insert(
            "proxy".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proxy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "proxy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_instanceable(mut self, val: &str) -> Self {
        self.params.insert(
            "instanceable".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_instanceable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instanceable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_visibility(mut self, val: &str) -> Self {
        self.params.insert(
            "visibility".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_visibility_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visibility".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extents(mut self, val: &str) -> Self {
        self.params.insert(
            "extents".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extents".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kind(mut self, val: &str) -> Self {
        self.params.insert(
            "kind".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_type(mut self, val: &str) -> Self {
        self.params.insert(
            "type".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "type".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier(mut self, val: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_apischemas(mut self, val: &str) -> Self {
        self.params.insert(
            "apischemas".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_apischemas_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apischemas".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_drawmode(mut self, val: &str) -> Self {
        self.params.insert(
            "drawmode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_drawmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "drawmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cardgeometry(mut self, val: &str) -> Self {
        self.params.insert(
            "cardgeometry".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cardgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cardgeometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cardtexturexneg(mut self, val: &str) -> Self {
        self.params.insert(
            "cardtexturexneg".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cardtexturexneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cardtexturexneg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cardtexturexpos(mut self, val: &str) -> Self {
        self.params.insert(
            "cardtexturexpos".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cardtexturexpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cardtexturexpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cardtextureyneg(mut self, val: &str) -> Self {
        self.params.insert(
            "cardtextureyneg".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cardtextureyneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cardtextureyneg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cardtextureypos(mut self, val: &str) -> Self {
        self.params.insert(
            "cardtextureypos".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cardtextureypos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cardtextureypos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cardtexturezneg(mut self, val: &str) -> Self {
        self.params.insert(
            "cardtexturezneg".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cardtexturezneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cardtexturezneg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cardtexturezpos(mut self, val: &str) -> Self {
        self.params.insert(
            "cardtexturezpos".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cardtexturezpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cardtexturezpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetidentifier(mut self, val: &str) -> Self {
        self.params.insert(
            "assetidentifier".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetidentifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "assetidentifier".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetname(mut self, val: &str) -> Self {
        self.params.insert(
            "assetname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "assetname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetthumbnail(mut self, val: &str) -> Self {
        self.params.insert(
            "assetthumbnail".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetthumbnail_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "assetthumbnail".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetversion(mut self, val: &str) -> Self {
        self.params.insert(
            "assetversion".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetversion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "assetversion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetdependencies(mut self, val: &str) -> Self {
        self.params.insert(
            "assetdependencies".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetdependencies_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "assetdependencies".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfoname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("assetinfoname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfoname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("assetinfoname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfotype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("assetinfotype{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfotype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("assetinfotype{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfostrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("assetinfostrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfostrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("assetinfostrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_icon(mut self, val: &str) -> Self {
        self.params.insert(
            "icon".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_icon_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "icon".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_catalogparentpath(mut self, val: &str) -> Self {
        self.params.insert(
            "catalogparentpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_catalogparentpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "catalogparentpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customdataname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdataname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatatype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customdatatype{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatatype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdatatype{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatastrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customdatastrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatastrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdatastrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setpurpose(mut self, val: bool) -> Self {
        self.params.insert(
            "setpurpose".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setpurpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setpurpose".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setproxy(mut self, val: bool) -> Self {
        self.params.insert(
            "setproxy".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setproxy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setproxy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setinstanceable(mut self, val: bool) -> Self {
        self.params.insert(
            "setinstanceable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setinstanceable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setinstanceable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setactivation(mut self, val: bool) -> Self {
        self.params.insert(
            "setactivation".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setactivation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setactivation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: bool) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setvisibility(mut self, val: bool) -> Self {
        self.params.insert(
            "setvisibility".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setvisibility_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setvisibility".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setextents(mut self, val: bool) -> Self {
        self.params.insert(
            "setextents".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setextents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setextents".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setkind(mut self, val: bool) -> Self {
        self.params.insert(
            "setkind".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setkind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fixkindhierarchy(mut self, val: bool) -> Self {
        self.params.insert(
            "fixkindhierarchy".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fixkindhierarchy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fixkindhierarchy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_settype(mut self, val: bool) -> Self {
        self.params.insert(
            "settype".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_settype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "settype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setspecifier(mut self, val: bool) -> Self {
        self.params.insert(
            "setspecifier".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setspecifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setspecifier".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setapischemas(mut self, val: bool) -> Self {
        self.params.insert(
            "setapischemas".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setapischemas_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setapischemas".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setapplydrawmode(mut self, val: bool) -> Self {
        self.params.insert(
            "setapplydrawmode".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setapplydrawmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setapplydrawmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_applydrawmode(mut self, val: bool) -> Self {
        self.params.insert(
            "applydrawmode".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_applydrawmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "applydrawmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setdrawmode(mut self, val: bool) -> Self {
        self.params.insert(
            "setdrawmode".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setdrawmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setdrawmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setdrawmodecolor(mut self, val: bool) -> Self {
        self.params.insert(
            "setdrawmodecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setdrawmodecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setdrawmodecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setcardgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "setcardgeometry".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setcardgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setcardgeometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setcardtexturexneg(mut self, val: bool) -> Self {
        self.params.insert(
            "setcardtexturexneg".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setcardtexturexneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setcardtexturexneg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setcardtexturexpos(mut self, val: bool) -> Self {
        self.params.insert(
            "setcardtexturexpos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setcardtexturexpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setcardtexturexpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setcardtextureyneg(mut self, val: bool) -> Self {
        self.params.insert(
            "setcardtextureyneg".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setcardtextureyneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setcardtextureyneg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setcardtextureypos(mut self, val: bool) -> Self {
        self.params.insert(
            "setcardtextureypos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setcardtextureypos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setcardtextureypos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setcardtexturezneg(mut self, val: bool) -> Self {
        self.params.insert(
            "setcardtexturezneg".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setcardtexturezneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setcardtexturezneg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setcardtexturezpos(mut self, val: bool) -> Self {
        self.params.insert(
            "setcardtexturezpos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setcardtexturezpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setcardtexturezpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setassetidentifier(mut self, val: bool) -> Self {
        self.params.insert(
            "setassetidentifier".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setassetidentifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setassetidentifier".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setassetname(mut self, val: bool) -> Self {
        self.params.insert(
            "setassetname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setassetname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setassetname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setassetthumbnail(mut self, val: bool) -> Self {
        self.params.insert(
            "setassetthumbnail".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setassetthumbnail_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setassetthumbnail".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setassetversion(mut self, val: bool) -> Self {
        self.params.insert(
            "setassetversion".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setassetversion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setassetversion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setassetdependencies(mut self, val: bool) -> Self {
        self.params.insert(
            "setassetdependencies".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setassetdependencies_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setassetdependencies".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfoboolvalue_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("assetinfoboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_assetinfoboolvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("assetinfoboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_seticon(mut self, val: bool) -> Self {
        self.params.insert(
            "seticon".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_seticon_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seticon".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setcatalogparentpath(mut self, val: bool) -> Self {
        self.params.insert(
            "setcatalogparentpath".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setcatalogparentpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setcatalogparentpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_seteditable(mut self, val: bool) -> Self {
        self.params.insert(
            "seteditable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_seteditable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seteditable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_editable(mut self, val: bool) -> Self {
        self.params.insert(
            "editable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_editable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "editable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setselectable(mut self, val: bool) -> Self {
        self.params.insert(
            "setselectable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setselectable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setselectable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_selectable(mut self, val: bool) -> Self {
        self.params.insert(
            "selectable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_selectable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "selectable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sethideinui(mut self, val: bool) -> Self {
        self.params.insert(
            "sethideinui".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sethideinui_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sethideinui".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hideinui(mut self, val: bool) -> Self {
        self.params.insert(
            "hideinui".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hideinui_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hideinui".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataboolvalue_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("customdataboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_customdataboolvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdataboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopConfigureprimitive {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "configureprimitive"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait LopConfigureprimitiveOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopConfigureprimitiveOutputs for LopConfigureprimitive {}
impl LopConfigureprimitiveOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<LopConfigureprimitive>
{
}

#[derive(Debug, Clone)]
pub struct LopConfigureproperty {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopConfigureproperty {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_customdatafloatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("customdatafloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_customdatafloatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdatafloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfofloatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("assetinfofloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_assetinfofloatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("assetinfofloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_elementsize(mut self, val: i32) -> Self {
        self.params.insert(
            "elementsize".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_elementsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataintvalue_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("customdataintvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_customdataintvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdataintvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfointvalue_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("assetinfointvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_assetinfointvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("assetinfointvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proppattern(mut self, val: &str) -> Self {
        self.params.insert(
            "proppattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proppattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "proppattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variability(mut self, val: &str) -> Self {
        self.params.insert(
            "variability".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variability_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variability".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_colorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "colorspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_colorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_interpolation(mut self, val: &str) -> Self {
        self.params.insert(
            "interpolation".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_interpolation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interpolation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customdataname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdataname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatatype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customdatatype{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatatype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdatatype{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatastrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customdatastrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdatastrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdatastrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfoname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("assetinfoname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfoname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("assetinfoname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfotype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("assetinfotype{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfotype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("assetinfotype{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfostrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("assetinfostrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfostrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("assetinfostrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setvariability(mut self, val: bool) -> Self {
        self.params.insert(
            "setvariability".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setvariability_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setvariability".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setcolorspace(mut self, val: bool) -> Self {
        self.params.insert(
            "setcolorspace".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setcolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setcolorspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setinterpolation(mut self, val: bool) -> Self {
        self.params.insert(
            "setinterpolation".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setinterpolation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setinterpolation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setelementsize(mut self, val: bool) -> Self {
        self.params.insert(
            "setelementsize".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setelementsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setelementsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customdataboolvalue_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("customdataboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_customdataboolvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customdataboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_assetinfoboolvalue_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("assetinfoboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_assetinfoboolvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("assetinfoboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopConfigureproperty {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "configureproperty"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait LopConfigurepropertyOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopConfigurepropertyOutputs for LopConfigureproperty {}
impl LopConfigurepropertyOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<LopConfigureproperty>
{
}

#[derive(Debug, Clone)]
pub struct LopConfigurestage {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopConfigurestage {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_stagevariablesfloatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("stagevariablesfloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stagevariablesfloatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stagevariablesfloatvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablesintvalue_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("stagevariablesintvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stagevariablesintvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stagevariablesintvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_editpopulate(mut self, val: &str) -> Self {
        self.params.insert(
            "editpopulate".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_editpopulate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "editpopulate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_populatepattern(mut self, val: &str) -> Self {
        self.params.insert(
            "populatepattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_populatepattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "populatepattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unpopulatepattern(mut self, val: &str) -> Self {
        self.params.insert(
            "unpopulatepattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unpopulatepattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unpopulatepattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_populatepaths(mut self, val: &str) -> Self {
        self.params.insert(
            "populatepaths".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_populatepaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "populatepaths".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unpopulatepaths(mut self, val: &str) -> Self {
        self.params.insert(
            "unpopulatepaths".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unpopulatepaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unpopulatepaths".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_editload(mut self, val: &str) -> Self {
        self.params.insert(
            "editload".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_editload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "editload".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loadpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "loadpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loadpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loadpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unloadpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "unloadpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unloadpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unloadpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loadpaths(mut self, val: &str) -> Self {
        self.params.insert(
            "loadpaths".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loadpaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loadpaths".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unloadpaths(mut self, val: &str) -> Self {
        self.params.insert(
            "unloadpaths".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unloadpaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unloadpaths".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_editmute(mut self, val: &str) -> Self {
        self.params.insert(
            "editmute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_editmute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "editmute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mutepaths(mut self, val: &str) -> Self {
        self.params.insert(
            "mutepaths".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mutepaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mutepaths".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unmutepaths(mut self, val: &str) -> Self {
        self.params.insert(
            "unmutepaths".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unmutepaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unmutepaths".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resolvercontextassetpath(mut self, val: &str) -> Self {
        self.params.insert(
            "resolvercontextassetpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resolvercontextassetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolvercontextassetpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resolvercontextstringurlprefix_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("resolvercontextstringurlprefix{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resolvercontextstringurlprefix_inst_expr(
        mut self,
        index1: usize,
        expr: &str,
    ) -> Self {
        self.params.insert(
            format!("resolvercontextstringurlprefix{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resolvercontextstringvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("resolvercontextstringvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resolvercontextstringvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resolvercontextstringvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantselectionset_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("variantselectionset{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantselectionset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantselectionset{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantselectionvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("variantselectionvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantselectionvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantselectionvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablesname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("stagevariablesname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablesname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stagevariablesname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablestype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("stagevariablestype{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablestype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stagevariablestype{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablesstrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("stagevariablesstrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablesstrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stagevariablesstrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resolvercontextstringenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("resolvercontextstringenable{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resolvercontextstringenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resolvercontextstringenable{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantselectionenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("variantselectionenable{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_variantselectionenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantselectionenable{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stagevariablesboolvalue_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("stagevariablesboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stagevariablesboolvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stagevariablesboolvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopConfigurestage {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "configurestage"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait LopConfigurestageOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopConfigurestageOutputs for LopConfigurestage {}
impl LopConfigurestageOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<LopConfigurestage>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCoordsysTargettype {
    Xform = 0,
    Camera = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCoordsysXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCoordsysRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopCoordsys {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopCoordsys {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_zoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "zoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_focallength(mut self, val: f32) -> Self {
        self.params.insert(
            "focalLength".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_focallength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "focalLength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidescale(mut self, val: f32) -> Self {
        self.params.insert(
            "guidescale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guidescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidescale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aspectratio(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "aspectratio".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_aspectratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aspectratio".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shear".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shear".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targettype(mut self, val: LopCoordsysTargettype) -> Self {
        self.params.insert(
            "targettype".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_targettype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targettype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xord(mut self, val: LopCoordsysXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rord(mut self, val: LopCoordsysRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "resMenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_resmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resMenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_projection(mut self, val: &str) -> Self {
        self.params.insert(
            "projection".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_projection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projection".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("primpattern{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("primpattern{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mode_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("mode{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mode_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("mode{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_target_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("target{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_target_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("target{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_createtarget(mut self, val: bool) -> Self {
        self.params.insert(
            "createtarget".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createtarget_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createtarget".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ignoreparentxforms(mut self, val: bool) -> Self {
        self.params.insert(
            "ignoreparentxforms".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ignoreparentxforms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ignoreparentxforms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inviewermenu(mut self, val: bool) -> Self {
        self.params.insert(
            "inviewermenu".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inviewermenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inviewermenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopCoordsys {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "coordsys"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait LopCoordsysOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopCoordsysOutputs for LopCoordsys {}
impl LopCoordsysOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<LopCoordsys> {}

#[derive(Debug, Clone)]
pub struct LopCop2net {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopCop2net {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopCop2net {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "cop2net"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCopnetBorder {
    Constant = 0,
    Clamp = 1,
    Mirror = 2,
    Wrap = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCopnetPrecision {
    /// 16-bit
    N16MinusBit = 0,
    /// 32-bit
    N32MinusBit = 1,
}

#[derive(Debug, Clone)]
pub struct LopCopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopCopnet {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn with_pixelscale(mut self, val: f32) -> Self {
        self.params.insert(
            "pixelscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pixelscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_udim(mut self, val: i32) -> Self {
        self.params.insert(
            "udim".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_udim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "udim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vistile(mut self, val: i32) -> Self {
        self.params.insert(
            "vistile".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vistile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vistile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_res(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "res".to_string(),
            houdini_ramen_core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_res_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "resmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_resmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resmenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_border(mut self, val: LopCopnetBorder) -> Self {
        self.params.insert(
            "border".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_border_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "border".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_precision(mut self, val: LopCopnetPrecision) -> Self {
        self.params.insert(
            "precision".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_precision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "precision".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setres(mut self, val: bool) -> Self {
        self.params.insert(
            "setres".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setres".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setpixelscale(mut self, val: bool) -> Self {
        self.params.insert(
            "setpixelscale".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setpixelscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setpixelscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setborder(mut self, val: bool) -> Self {
        self.params.insert(
            "setborder".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setborder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setborder".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setprecision(mut self, val: bool) -> Self {
        self.params.insert(
            "setprecision".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setprecision".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setudim(mut self, val: bool) -> Self {
        self.params.insert(
            "setudim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setudim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setvistile(mut self, val: bool) -> Self {
        self.params.insert(
            "setvistile".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setvistile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setvistile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopCopnet {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "copnet"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct LopCopyproperty {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopCopyproperty {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destprimpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "destprimpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destprimpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destprimpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceprop_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("sourceprop{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceprop_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("sourceprop{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destprop_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("destprop{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destprop_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("destprop{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_separatedestprimpattern(mut self, val: bool) -> Self {
        self.params.insert(
            "separatedestprimpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_separatedestprimpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "separatedestprimpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_copymetadata_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("copymetadata{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_copymetadata_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("copymetadata{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blocksource_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("blocksource{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_blocksource_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("blocksource{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopCopyproperty {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "copyproperty"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait LopCopypropertyOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopCopypropertyOutputs for LopCopyproperty {}
impl LopCopypropertyOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<LopCopyproperty> {}

#[derive(Debug, Clone)]
pub struct LopCreatelod {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopCreatelod {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_lodpercent_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("lodpercent{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lodpercent_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("lodpercent{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_equalizelengths_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("equalizelengths{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_equalizelengths_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("equalizelengths{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_boundaryweight_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("boundaryweight{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_boundaryweight_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("boundaryweight{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_idoffset(mut self, val: i32) -> Self {
        self.params.insert(
            "idoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_idoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "idoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "sourceprimpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceprimpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind(mut self, val: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantset(mut self, val: &str) -> Self {
        self.params.insert(
            "variantset".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_varnameroot(mut self, val: &str) -> Self {
        self.params.insert(
            "varnameroot".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_varnameroot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "varnameroot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantname1(mut self, val: &str) -> Self {
        self.params.insert(
            "variantname1".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variantname1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantname1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_custvariantname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("custvariantname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_custvariantname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("custvariantname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceprim(mut self, val: bool) -> Self {
        self.params.insert(
            "sourceprim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sourceprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_checkopinions(mut self, val: bool) -> Self {
        self.params.insert(
            "checkopinions".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_checkopinions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkopinions".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usecustvarnames(mut self, val: bool) -> Self {
        self.params.insert(
            "usecustvarnames".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecustvarnames_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecustvarnames".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displayvariant(mut self, val: bool) -> Self {
        self.params.insert(
            "displayvariant".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayvariant_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayvariant".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_originalpoints_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("originalpoints{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_originalpoints_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("originalpoints{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_preservequads_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("preservequads{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_preservequads_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("preservequads{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopCreatelod {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "createlod"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait LopCreatelodOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopCreatelodOutputs for LopCreatelod {}
impl LopCreatelodOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<LopCreatelod> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCreatexformCreateprims {
    Edit = 0,
    Create = 1,
    /// Force Edit (Ignore Editable Flag)
    ForceEditIgnoreEditableFlag = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCreatexformXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCreatexformRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopCreatexform {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopCreatexform {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shear".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shear".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primcount(mut self, val: i32) -> Self {
        self.params.insert(
            "primcount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primcount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_createprims(mut self, val: LopCreatexformCreateprims) -> Self {
        self.params.insert(
            "createprims".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createprims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initforedit(mut self, val: i32) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_initforedit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xord(mut self, val: LopCreatexformXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rord(mut self, val: LopCreatexformRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primtype(mut self, val: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind(mut self, val: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier(mut self, val: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_classancestor(mut self, val: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_classancestor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopCreatexform {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "createxform"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait LopCreatexformOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopCreatexformOutputs for LopCreatexform {}
impl LopCreatexformOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<LopCreatexform> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCubeCreateprims {
    Edit = 0,
    Create = 1,
    /// Force Edit (Ignore Editable Flag)
    ForceEditIgnoreEditableFlag = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCubeXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCubeRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopCube {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopCube {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_size(mut self, val: f32) -> Self {
        self.params.insert(
            "size".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_ycb(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_ycb".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_ycb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_ycb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_p8a(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_p8a".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_p8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_p8a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shear".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shear".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primcount(mut self, val: i32) -> Self {
        self.params.insert(
            "primcount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primcount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_createprims(mut self, val: LopCubeCreateprims) -> Self {
        self.params.insert(
            "createprims".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createprims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initforedit(mut self, val: i32) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_initforedit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xord(mut self, val: LopCubeXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rord(mut self, val: LopCubeRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primtype(mut self, val: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind(mut self, val: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier(mut self, val: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_classancestor(mut self, val: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_classancestor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_size_control(mut self, val: &str) -> Self {
        self.params.insert(
            "size_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_size_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_control_qmb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_control_qmb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_control_qmb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_control_qmb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_control_zpb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_control_zpb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_control_zpb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_control_zpb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doublesided_control(mut self, val: &str) -> Self {
        self.params.insert(
            "doubleSided_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doublesided_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doubleSided_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computeextents(mut self, val: bool) -> Self {
        self.params.insert(
            "computeextents".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computeextents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computeextents".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doublesided(mut self, val: bool) -> Self {
        self.params.insert(
            "doubleSided".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doublesided_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doubleSided".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopCube {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "cube"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait LopCubeOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopCubeOutputs for LopCube {}
impl LopCubeOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<LopCube> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCylinderCreateprims {
    Edit = 0,
    Create = 1,
    /// Force Edit (Ignore Editable Flag)
    ForceEditIgnoreEditableFlag = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCylinderXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopCylinderRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopCylinder {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopCylinder {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_height(mut self, val: f32) -> Self {
        self.params.insert(
            "height".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_height_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "height".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_radiusbottom(mut self, val: f32) -> Self {
        self.params.insert(
            "radiusBottom".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radiusbottom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radiusBottom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_radiustop(mut self, val: f32) -> Self {
        self.params.insert(
            "radiusTop".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radiustop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radiusTop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_ycb(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_ycb".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_ycb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_ycb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_p8a(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_p8a".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_p8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_p8a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shear".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shear".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primcount(mut self, val: i32) -> Self {
        self.params.insert(
            "primcount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primcount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_createprims(mut self, val: LopCylinderCreateprims) -> Self {
        self.params.insert(
            "createprims".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createprims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initforedit(mut self, val: i32) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_initforedit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xord(mut self, val: LopCylinderXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rord(mut self, val: LopCylinderRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primtype(mut self, val: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind(mut self, val: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier(mut self, val: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_classancestor(mut self, val: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_classancestor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axis_control(mut self, val: &str) -> Self {
        self.params.insert(
            "axis_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axis_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axis_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axis(mut self, val: &str) -> Self {
        self.params.insert(
            "axis".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_height_control(mut self, val: &str) -> Self {
        self.params.insert(
            "height_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_height_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "height_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_radiusbottom_control(mut self, val: &str) -> Self {
        self.params.insert(
            "radiusBottom_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_radiusbottom_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radiusBottom_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_radiustop_control(mut self, val: &str) -> Self {
        self.params.insert(
            "radiusTop_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_radiustop_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radiusTop_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_control_qmb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_control_qmb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_control_qmb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_control_qmb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_control_zpb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_control_zpb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_control_zpb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_control_zpb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doublesided_control(mut self, val: &str) -> Self {
        self.params.insert(
            "doubleSided_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doublesided_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doubleSided_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computeextents(mut self, val: bool) -> Self {
        self.params.insert(
            "computeextents".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computeextents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computeextents".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doublesided(mut self, val: bool) -> Self {
        self.params.insert(
            "doubleSided".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doublesided_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doubleSided".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopCylinder {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "cylinder"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
        &self.inputs
    }
    fn get_params(
        &self,
    ) -> &std::collections::HashMap<String, houdini_ramen_core::types::ParamValue> {
        &self.params
    }
    fn get_spare_params(&self) -> &[houdini_ramen_core::types::SpareParam] {
        &self.spare_params
    }
}

pub trait LopCylinderOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopCylinderOutputs for LopCylinder {}
impl LopCylinderOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<LopCylinder> {}
