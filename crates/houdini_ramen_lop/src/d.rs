#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopDistantlightCreateprims {
    Edit = 0,
    Create = 1,
    /// Force Edit (Ignore Editable Flag)
    ForceEditIgnoreEditableFlag = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopDistantlightXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopDistantlightRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopDistantlight {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopDistantlight {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_distance(mut self, val: f32) -> Self {
        self.params.insert(
            "distance".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distance".to_string(),
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
    pub fn with_xn_inputsintensity_i0a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsintensity_i0a".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsintensity_i0a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsintensity_i0a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsexposure_vya(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsexposure_vya".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsexposure_vya_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsexposure_vya".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputscolortemperature_wcb(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputscolorTemperature_wcb".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputscolortemperature_wcb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputscolorTemperature_wcb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsangle_zta(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsangle_zta".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsangle_zta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsangle_zta".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsdiffuse_8wa(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsdiffuse_8wa".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsdiffuse_8wa_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsdiffuse_8wa".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsspecular_vya(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsspecular_vya".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsspecular_vya_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsspecular_vya".to_string(),
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
    pub fn with_spotlightdist(mut self, val: f32) -> Self {
        self.params.insert(
            "spotlightdist".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spotlightdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spotlightdist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingconeangle_wcbhe(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsshapingconeangle_wcbhe".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsshapingconeangle_wcbhe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingconeangle_wcbhe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingconesoftness_shbhe(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsshapingconesoftness_shbhe".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsshapingconesoftness_shbhe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingconesoftness_shbhe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorleft(mut self, val: f32) -> Self {
        self.params.insert(
            "barndoorleft".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barndoorleft_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorleft".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorleftedge(mut self, val: f32) -> Self {
        self.params.insert(
            "barndoorleftedge".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barndoorleftedge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorleftedge".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorright(mut self, val: f32) -> Self {
        self.params.insert(
            "barndoorright".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barndoorright_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorright".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorrightedge(mut self, val: f32) -> Self {
        self.params.insert(
            "barndoorrightedge".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barndoorrightedge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorrightedge".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoortop(mut self, val: f32) -> Self {
        self.params.insert(
            "barndoortop".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barndoortop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoortop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoortopedge(mut self, val: f32) -> Self {
        self.params.insert(
            "barndoortopedge".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barndoortopedge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoortopedge".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorbottom(mut self, val: f32) -> Self {
        self.params.insert(
            "barndoorbottom".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barndoorbottom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorbottom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorbottomedge(mut self, val: f32) -> Self {
        self.params.insert(
            "barndoorbottomedge".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barndoorbottomedge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorbottomedge".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingfocus_e5ah(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsshapingfocus_e5ah".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsshapingfocus_e5ah_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingfocus_e5ah".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingiesanglescale_fjbhd(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsshapingiesangleScale_fjbhd".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsshapingiesanglescale_fjbhd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesangleScale_fjbhd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowfalloff_06ag(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsshadowfalloff_06ag".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsshadowfalloff_06ag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowfalloff_06ag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowfalloffgamma_5fbg(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsshadowfalloffGamma_5fbg".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsshadowfalloffgamma_5fbg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowfalloffGamma_5fbg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowdistance_n8ag(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsshadowdistance_n8ag".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsshadowdistance_n8ag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowdistance_n8ag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_handlefocus(mut self, val: f32) -> Self {
        self.params.insert(
            "handlefocus".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_handlefocus_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handlefocus".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
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
    pub fn with_xn_houdiniclippingrange_o8a(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "xn__houdiniclippingRange_o8a".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_xn_houdiniclippingrange_o8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniclippingRange_o8a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
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
    pub fn with_hitpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hitpos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hitpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hitnormal(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "hitnormal".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hitnormal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitnormal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadowpivot(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shadowpivot".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shadowpivot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowpivot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shadowtarget(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shadowtarget".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shadowtarget_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowtarget".to_string(),
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
    pub fn with_xn_inputscolor_zta(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__inputscolor_zta".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_inputscolor_zta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputscolor_zta".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingfocustint_wcbh(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__inputsshapingfocusTint_wcbh".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_inputsshapingfocustint_wcbh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingfocusTint_wcbh".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowcolor_r3ag(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__inputsshadowcolor_r3ag".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_inputsshadowcolor_r3ag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowcolor_r3ag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
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

    // --- Menu parameters ---
    pub fn with_createprims(mut self, val: LopDistantlightCreateprims) -> Self {
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
    pub fn with_xord(mut self, val: LopDistantlightXord) -> Self {
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
    pub fn with_rord(mut self, val: LopDistantlightRord) -> Self {
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

    // --- String parameters ---
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
    pub fn with_xn_inputsintensity_control_jeb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsintensity_control_jeb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsintensity_control_jeb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsintensity_control_jeb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsexposure_control_wcb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsexposure_control_wcb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsexposure_control_wcb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsexposure_control_wcb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputscolor_control_06a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputscolor_control_06a".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputscolor_control_06a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputscolor_control_06a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsenablecolortemperature_control_pzb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsenableColorTemperature_control_pzb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsenablecolortemperature_control_pzb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsenableColorTemperature_control_pzb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputscolortemperature_control_xpb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputscolorTemperature_control_xpb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputscolortemperature_control_xpb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputscolorTemperature_control_xpb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsangle_control_06a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsangle_control_06a".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsangle_control_06a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsangle_control_06a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_houdiniclippingrange_control_pmb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__houdiniclippingRange_control_pmb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_houdiniclippingrange_control_pmb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniclippingRange_control_pmb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsnormalize_control_jeb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsnormalize_control_jeb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsnormalize_control_jeb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsnormalize_control_jeb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsdiffuse_control_99a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsdiffuse_control_99a".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsdiffuse_control_99a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsdiffuse_control_99a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsspecular_control_wcb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsspecular_control_wcb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsspecular_control_wcb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsspecular_control_wcb".to_string(),
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
    pub fn with_xn_lightfilters_control_m8a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__lightfilters_control_m8a".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_lightfilters_control_m8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__lightfilters_control_m8a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_lightfilters_lva(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__lightfilters_lva".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_lightfilters_lva_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__lightfilters_lva".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingconeangle_control_xpbhe(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingconeangle_control_xpbhe".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingconeangle_control_xpbhe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingconeangle_control_xpbhe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingconesoftness_control_tubhe(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingconesoftness_control_tubhe".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingconesoftness_control_tubhe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingconesoftness_control_tubhe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorleft_control(mut self, val: &str) -> Self {
        self.params.insert(
            "barndoorleft_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorleft_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorleft_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorleftedge_control(mut self, val: &str) -> Self {
        self.params.insert(
            "barndoorleftedge_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorleftedge_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorleftedge_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorright_control(mut self, val: &str) -> Self {
        self.params.insert(
            "barndoorright_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorright_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorright_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorrightedge_control(mut self, val: &str) -> Self {
        self.params.insert(
            "barndoorrightedge_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorrightedge_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorrightedge_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoortop_control(mut self, val: &str) -> Self {
        self.params.insert(
            "barndoortop_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoortop_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoortop_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoortopedge_control(mut self, val: &str) -> Self {
        self.params.insert(
            "barndoortopedge_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoortopedge_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoortopedge_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorbottom_control(mut self, val: &str) -> Self {
        self.params.insert(
            "barndoorbottom_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorbottom_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorbottom_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorbottomedge_control(mut self, val: &str) -> Self {
        self.params.insert(
            "barndoorbottomedge_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barndoorbottomedge_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barndoorbottomedge_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingfocus_control_fjbh(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingfocus_control_fjbh".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingfocus_control_fjbh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingfocus_control_fjbh".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingfocustint_control_xpbh(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingfocusTint_control_xpbh".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingfocustint_control_xpbh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingfocusTint_control_xpbh".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingiesfile_control_ombhd(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesfile_control_ombhd".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingiesfile_control_ombhd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesfile_control_ombhd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingiesfile_n8ahd(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesfile_n8ahd".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingiesfile_n8ahd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesfile_n8ahd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingiesnormalize_control_tubhd(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesnormalize_control_tubhd".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingiesnormalize_control_tubhd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesnormalize_control_tubhd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingiesanglescale_control_gwbhd(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesangleScale_control_gwbhd".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingiesanglescale_control_gwbhd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesangleScale_control_gwbhd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowenable_control_fjbg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowenable_control_fjbg".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowenable_control_fjbg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowenable_control_fjbg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowcolor_control_shbg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowcolor_control_shbg".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowcolor_control_shbg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowcolor_control_shbg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowfalloff_control_1kbg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowfalloff_control_1kbg".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowfalloff_control_1kbg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowfalloff_control_1kbg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowfalloffgamma_control_6sbg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowfalloffGamma_control_6sbg".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowfalloffgamma_control_6sbg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowfalloffGamma_control_6sbg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowdistance_control_ombg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowdistance_control_ombg".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowdistance_control_ombg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowdistance_control_ombg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
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
    pub fn with_hit(mut self, val: bool) -> Self {
        self.params.insert(
            "hit".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hasshadowpivot(mut self, val: bool) -> Self {
        self.params.insert(
            "hasshadowpivot".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hasshadowpivot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hasshadowpivot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hasshadowtarget(mut self, val: bool) -> Self {
        self.params.insert(
            "hasshadowtarget".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hasshadowtarget_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hasshadowtarget".to_string(),
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
    pub fn with_xn_inputsenablecolortemperature_omb(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__inputsenableColorTemperature_omb".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_inputsenablecolortemperature_omb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsenableColorTemperature_omb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsnormalize_i0a(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__inputsnormalize_i0a".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_inputsnormalize_i0a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsnormalize_i0a".to_string(),
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
    pub fn with_spotlightenable(mut self, val: bool) -> Self {
        self.params.insert(
            "spotlightenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_spotlightenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spotlightenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_focusenable(mut self, val: bool) -> Self {
        self.params.insert(
            "focusenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_focusenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "focusenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iesenable(mut self, val: bool) -> Self {
        self.params.insert(
            "iesenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_iesenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iesenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshapingiesnormalize_shbhd(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__inputsshapingiesnormalize_shbhd".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_inputsshapingiesnormalize_shbhd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshapingiesnormalize_shbhd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowenable_e5ag(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__inputsshadowenable_e5ag".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_inputsshadowenable_e5ag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowenable_e5ag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopDistantlight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "distantlight"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum LopDomelightCreateprims {
    Edit = 0,
    Create = 1,
    /// Force Edit (Ignore Editable Flag)
    ForceEditIgnoreEditableFlag = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopDomelightXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopDomelightRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopDomelight {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopDomelight {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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
    pub fn with_xn_inputsintensity_i0a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsintensity_i0a".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsintensity_i0a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsintensity_i0a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsexposure_vya(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsexposure_vya".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsexposure_vya_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsexposure_vya".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputscolortemperature_wcb(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputscolorTemperature_wcb".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputscolortemperature_wcb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputscolorTemperature_wcb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsdiffuse_8wa(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsdiffuse_8wa".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsdiffuse_8wa_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsdiffuse_8wa".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsspecular_vya(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsspecular_vya".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsspecular_vya_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsspecular_vya".to_string(),
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
    pub fn with_xn_inputsshadowdistance_n8ag(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsshadowdistance_n8ag".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsshadowdistance_n8ag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowdistance_n8ag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
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

    // --- Float3 parameters ---
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
    pub fn with_xn_inputscolor_zta(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__inputscolor_zta".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_inputscolor_zta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputscolor_zta".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowcolor_r3ag(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__inputsshadowcolor_r3ag".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_inputsshadowcolor_r3ag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowcolor_r3ag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
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

    // --- Menu parameters ---
    pub fn with_createprims(mut self, val: LopDomelightCreateprims) -> Self {
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
    pub fn with_xord(mut self, val: LopDomelightXord) -> Self {
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
    pub fn with_rord(mut self, val: LopDomelightRord) -> Self {
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

    // --- String parameters ---
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
    pub fn with_xn_inputsintensity_control_jeb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsintensity_control_jeb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsintensity_control_jeb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsintensity_control_jeb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsexposure_control_wcb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsexposure_control_wcb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsexposure_control_wcb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsexposure_control_wcb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputscolor_control_06a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputscolor_control_06a".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputscolor_control_06a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputscolor_control_06a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsenablecolortemperature_control_pzb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsenableColorTemperature_control_pzb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsenablecolortemperature_control_pzb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsenableColorTemperature_control_pzb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputscolortemperature_control_xpb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputscolorTemperature_control_xpb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputscolortemperature_control_xpb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputscolorTemperature_control_xpb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputstexturefile_control_shbh(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputstexturefile_control_shbh".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputstexturefile_control_shbh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputstexturefile_control_shbh".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputstexturefile_r3ah(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputstexturefile_r3ah".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputstexturefile_r3ah_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputstexturefile_r3ah".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputstextureformat_control_1kbh(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputstextureformat_control_1kbh".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputstextureformat_control_1kbh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputstextureformat_control_1kbh".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputstextureformat_06ah(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputstextureformat_06ah".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputstextureformat_06ah_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputstextureformat_06ah".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filters_control(mut self, val: &str) -> Self {
        self.params.insert(
            "filters_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filters_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filters_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filters(mut self, val: &str) -> Self {
        self.params.insert(
            "filters".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filters_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filters".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_portals_control(mut self, val: &str) -> Self {
        self.params.insert(
            "portals_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_portals_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "portals_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_portals(mut self, val: &str) -> Self {
        self.params.insert(
            "portals".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_portals_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "portals".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_poleaxis_control(mut self, val: &str) -> Self {
        self.params.insert(
            "poleAxis_control".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_poleaxis_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "poleAxis_control".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_poleaxis(mut self, val: &str) -> Self {
        self.params.insert(
            "poleAxis".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_poleaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "poleAxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsnormalize_control_jeb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsnormalize_control_jeb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsnormalize_control_jeb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsnormalize_control_jeb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsdiffuse_control_99a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsdiffuse_control_99a".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsdiffuse_control_99a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsdiffuse_control_99a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsspecular_control_wcb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsspecular_control_wcb".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsspecular_control_wcb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsspecular_control_wcb".to_string(),
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
    pub fn with_xn_lightfilters_control_m8a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__lightfilters_control_m8a".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_lightfilters_control_m8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__lightfilters_control_m8a".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_lightfilters_lva(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__lightfilters_lva".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_lightfilters_lva_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__lightfilters_lva".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowenable_control_fjbg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowenable_control_fjbg".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowenable_control_fjbg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowenable_control_fjbg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowcolor_control_shbg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowcolor_control_shbg".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowcolor_control_shbg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowcolor_control_shbg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowdistance_control_ombg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowdistance_control_ombg".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsshadowdistance_control_ombg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowdistance_control_ombg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
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
    pub fn with_xn_inputsenablecolortemperature_omb(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__inputsenableColorTemperature_omb".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_inputsenablecolortemperature_omb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsenableColorTemperature_omb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xn_inputsnormalize_i0a(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__inputsnormalize_i0a".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_inputsnormalize_i0a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsnormalize_i0a".to_string(),
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
    pub fn with_xn_inputsshadowenable_e5ag(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__inputsshadowenable_e5ag".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_inputsshadowenable_e5ag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowenable_e5ag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopDomelight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "domelight"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum LopDopnetCompresssims {
    NoCompression = 0,
    Blosc = 1,
    Gzip = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopDopnetXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopDopnetRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopDopnetPreXform {
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
pub enum LopDopnetUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct LopDopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopDopnet {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_resimulate(mut self) -> Self {
        self.params.insert(
            "resimulate".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_timestep(mut self, val: f32) -> Self {
        self.params.insert(
            "timestep".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timestep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timestep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_timeoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "timeoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timeoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_timescale(mut self, val: f32) -> Self {
        self.params.insert(
            "timescale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timescale".to_string(),
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
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert(
            "roll".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roll".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
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
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_substep(mut self, val: i32) -> Self {
        self.params.insert(
            "substep".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "substep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_startframe(mut self, val: i32) -> Self {
        self.params.insert(
            "startframe".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_startframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "startframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxfeedback(mut self, val: i32) -> Self {
        self.params.insert(
            "maxfeedback".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxfeedback_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxfeedback".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cachemaxsize(mut self, val: i32) -> Self {
        self.params.insert(
            "cachemaxsize".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachemaxsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachemaxsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_continuouscook_tick(mut self, val: i32) -> Self {
        self.params.insert(
            "continuouscook_tick".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_continuouscook_tick_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "continuouscook_tick".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_explicitcachensteps(mut self, val: i32) -> Self {
        self.params.insert(
            "explicitcachensteps".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_explicitcachensteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "explicitcachensteps".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_explicitcachecheckpointspacing(mut self, val: i32) -> Self {
        self.params.insert(
            "explicitcachecheckpointspacing".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_explicitcachecheckpointspacing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "explicitcachecheckpointspacing".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_compresssims(mut self, val: LopDopnetCompresssims) -> Self {
        self.params.insert(
            "compresssims".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_compresssims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compresssims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xord(mut self, val: LopDopnetXord) -> Self {
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
    pub fn with_rord(mut self, val: LopDopnetRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: LopDopnetPreXform) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: LopDopnetUparmtype) -> Self {
        self.params.insert(
            "uparmtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uparmtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_playfilesname(mut self, val: &str) -> Self {
        self.params.insert(
            "playfilesname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_playfilesname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "playfilesname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initialstate(mut self, val: &str) -> Self {
        self.params.insert(
            "initialstate".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initialstate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initialstate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_explicitcachename(mut self, val: &str) -> Self {
        self.params.insert(
            "explicitcachename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_explicitcachename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "explicitcachename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displayfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "displayfilter".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displayfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayfilter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_isplayer(mut self, val: bool) -> Self {
        self.params.insert(
            "isplayer".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_isplayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "isplayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_autoresim(mut self, val: bool) -> Self {
        self.params.insert(
            "autoresim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_autoresim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "autoresim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_datahints(mut self, val: bool) -> Self {
        self.params.insert(
            "datahints".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_datahints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "datahints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_interpolate(mut self, val: bool) -> Self {
        self.params.insert(
            "interpolate".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_interpolate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interpolate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cacheenabled(mut self, val: bool) -> Self {
        self.params.insert(
            "cacheenabled".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cacheenabled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cacheenabled".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cachetodisk(mut self, val: bool) -> Self {
        self.params.insert(
            "cachetodisk".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cachetodisk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachetodisk".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cachetodisknoninteractive(mut self, val: bool) -> Self {
        self.params.insert(
            "cachetodisknoninteractive".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cachetodisknoninteractive_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachetodisknoninteractive".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cachesubsteps(mut self, val: bool) -> Self {
        self.params.insert(
            "cachesubsteps".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cachesubsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachesubsteps".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_timeless(mut self, val: bool) -> Self {
        self.params.insert(
            "timeless".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeless_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeless".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_continuouscook(mut self, val: bool) -> Self {
        self.params.insert(
            "continuouscook".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_continuouscook_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "continuouscook".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_explicitcache(mut self, val: bool) -> Self {
        self.params.insert(
            "explicitcache".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_explicitcache_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "explicitcache".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopDopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "dopnet"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct LopDrawmode {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopDrawmode {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Additional Inputs for Render"
    pub fn set_input_additional_inputs_for_render<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Additional Inputs for Render" and specifies the output index of the target node.
    pub fn set_input_additional_inputs_for_render_from<
        N: houdini_ramen_core::types::HoudiniNode,
    >(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert(
            "execute".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_cookoutputnode(mut self) -> Self {
        self.params.insert(
            "cookoutputnode".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_cancelcook(mut self) -> Self {
        self.params.insert(
            "cancelcook".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_dirtyall(mut self) -> Self {
        self.params.insert(
            "dirtyall".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_domelightintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "domelightintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_domelightintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domelightintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_toplightintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "toplightintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_toplightintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "toplightintensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
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

    // --- Float3 parameters ---
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
    pub fn with_domelightcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "domelightcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_domelightcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domelightcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_toplightcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "toplightcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_toplightcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "toplightcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_toplightrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "toplightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_toplightrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "toplightrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
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
    pub fn with_maxres(mut self, val: i32) -> Self {
        self.params.insert(
            "maxres".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxres".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_targettopnetwork(mut self, val: &str) -> Self {
        self.params.insert(
            "targettopnetwork".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targettopnetwork_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targettopnetwork".to_string(),
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
    pub fn with_domelighttexture(mut self, val: &str) -> Self {
        self.params.insert(
            "domelighttexture".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_domelighttexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domelighttexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.params.insert(
            "topscheduler".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topscheduler".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
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
    pub fn with_domelight(mut self, val: bool) -> Self {
        self.params.insert(
            "domelight".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domelight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domelight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_toplight(mut self, val: bool) -> Self {
        self.params.insert(
            "toplight".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_toplight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "toplight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopDrawmode {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "drawmode"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum LopDropSplit {
    Component = 0,
    LeafPrimitive = 1,
    Group = 2,
    NoTraversal = 3,
}

#[derive(Debug, Clone)]
pub struct LopDrop {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopDrop {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_stashtransforms(mut self) -> Self {
        self.params.insert(
            "stashtransforms".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_deletestashedxforms(mut self) -> Self {
        self.params.insert(
            "deletestashedxforms".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_resimulate(mut self) -> Self {
        self.params.insert(
            "resimulate".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Data parameters ---
    pub fn with_stashedxforms(mut self, val: &str) -> Self {
        self.params.insert(
            "stashedxforms".to_string(),
            houdini_ramen_core::types::ParamValue::Data(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stashedxforms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stashedxforms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_collisionpadding(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionpadding".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionpadding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionpadding".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lift(mut self, val: f32) -> Self {
        self.params.insert(
            "lift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_separation(mut self, val: f32) -> Self {
        self.params.insert(
            "separation".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_separation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "separation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lineardrag(mut self, val: f32) -> Self {
        self.params.insert(
            "lineardrag".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lineardrag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lineardrag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_torquedrag(mut self, val: f32) -> Self {
        self.params.insert(
            "torquedrag".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_torquedrag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "torquedrag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_friction(mut self, val: f32) -> Self {
        self.params.insert(
            "friction".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_friction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "friction".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gravity(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gravity".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gravity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gravity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_split(mut self, val: LopDropSplit) -> Self {
        self.params.insert(
            "split".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_split_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "split".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_georep(mut self, val: &str) -> Self {
        self.params.insert(
            "georep".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_georep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "georep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collprimpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "collprimpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collprimpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collprimpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_passivegeorep(mut self, val: &str) -> Self {
        self.params.insert(
            "passivegeorep".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_passivegeorep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "passivegeorep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_pruneotherprims(mut self, val: bool) -> Self {
        self.params.insert(
            "pruneotherprims".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pruneotherprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pruneotherprims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usestashedxforms(mut self, val: bool) -> Self {
        self.params.insert(
            "usestashedxforms".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usestashedxforms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usestashedxforms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopDrop {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "drop"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
        Some("sopnet/dopnet")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait LopDropInnerExt {
    fn drag1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn geo2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn gravity1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn merge1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn merge2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn output(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rigidbodysolver1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> LopDropInnerExt for houdini_ramen_core::graph::InnerGraph<'a, LopDrop> {
    fn drag1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sopnet/dopnet/drag1")
    }
    fn geo1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sopnet/dopnet/geo1")
    }
    fn geo2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sopnet/dopnet/geo2")
    }
    fn gravity1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sopnet/dopnet/gravity1")
    }
    fn merge1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sopnet/dopnet/merge1")
    }
    fn merge2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sopnet/dopnet/merge2")
    }
    fn output(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sopnet/dopnet/output")
    }
    fn rigidbodysolver1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("sopnet/dopnet/rigidbodysolver1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopDuplicateXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopDuplicateRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopDuplicate {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopDuplicate {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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

    // --- Float3 parameters ---
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

    // --- Int parameters ---
    pub fn with_ncy(mut self, val: i32) -> Self {
        self.params.insert(
            "ncy".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ncy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ncy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_xord(mut self, val: LopDuplicateXord) -> Self {
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
    pub fn with_rord(mut self, val: LopDuplicateRord) -> Self {
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

    // --- String parameters ---
    pub fn with_sourceprims(mut self, val: &str) -> Self {
        self.params.insert(
            "sourceprims".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourceprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceprims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modifysource(mut self, val: &str) -> Self {
        self.params.insert(
            "modifysource".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_modifysource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modifysource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destinationprims(mut self, val: &str) -> Self {
        self.params.insert(
            "destinationprims".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destinationprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destinationprims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_duplicatename(mut self, val: &str) -> Self {
        self.params.insert(
            "duplicatename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_duplicatename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "duplicatename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_reftype(mut self, val: &str) -> Self {
        self.params.insert(
            "reftype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_reftype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reftype".to_string(),
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
    pub fn with_collectionpath(mut self, val: &str) -> Self {
        self.params.insert(
            "collectionpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collectionpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collectionpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collectionname(mut self, val: &str) -> Self {
        self.params.insert(
            "collectionname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collectionname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collectionname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_promotecollections(mut self, val: &str) -> Self {
        self.params.insert(
            "promotecollections".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_promotecollections_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "promotecollections".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_separatesource(mut self, val: bool) -> Self {
        self.params.insert(
            "separatesource".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_separatesource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "separatesource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_makeinstances(mut self, val: bool) -> Self {
        self.params.insert(
            "makeinstances".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_makeinstances_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "makeinstances".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_promotesourcecollections(mut self, val: bool) -> Self {
        self.params.insert(
            "promotesourcecollections".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_promotesourcecollections_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "promotesourcecollections".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xformcumulative(mut self, val: bool) -> Self {
        self.params.insert(
            "xformcumulative".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xformcumulative_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformcumulative".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopDuplicate {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "duplicate"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
