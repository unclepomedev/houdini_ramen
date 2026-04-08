#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverKarmaTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
    RenderFrameRangeFromStage = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverKarmaMblur {
    OnByDefault = 0,
    OffByDefault = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverKarmaDcmofsize {
    Monochrome = 0,
    FullColor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverKarmaPreviewscale {
    /// 100 %
    N100 = 0,
    /// 75 %
    N75 = 1,
    /// 66 %
    N66 = 2,
    /// 50 %
    N50 = 3,
    /// 33 %
    N33 = 4,
    /// 25 %
    N25 = 5,
    /// 20 %
    N20 = 6,
    /// 10 %
    N10 = 7,
}

#[derive(Debug, Clone)]
pub struct DriverKarma {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DriverKarma {
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

    /// Connects to input 0: "Input #1"
    pub fn set_input_input_1<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input #1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
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
    pub fn trigger_karmaview(mut self) -> Self {
        self.params.insert(
            "karmaview".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_render(mut self) -> Self {
        self.params.insert(
            "render".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_mplay(mut self) -> Self {
        self.params.insert(
            "mplay".to_string(),
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
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_head_ao_distance(mut self, val: f32) -> Self {
        self.params.insert(
            "head_ao_distance".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_ao_distance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_ao_distance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_depthcue_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "head_depthcue_alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_depthcue_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_depthcue_alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffusequality(mut self, val: f32) -> Self {
        self.params.insert(
            "diffusequality".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diffusequality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffusequality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflectquality(mut self, val: f32) -> Self {
        self.params.insert(
            "reflectquality".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflectquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflectquality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractquality(mut self, val: f32) -> Self {
        self.params.insert(
            "refractquality".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refractquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractquality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumequality(mut self, val: f32) -> Self {
        self.params.insert(
            "volumequality".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumequality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumequality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sssquality(mut self, val: f32) -> Self {
        self.params.insert(
            "sssquality".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sssquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sssquality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_sampling_quality(mut self, val: f32) -> Self {
        self.params.insert(
            "light_sampling_quality".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_light_sampling_quality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_sampling_quality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumesteprate(mut self, val: f32) -> Self {
        self.params.insert(
            "volumesteprate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumesteprate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumesteprate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuselimit(mut self, val: f32) -> Self {
        self.params.insert(
            "diffuselimit".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diffuselimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuselimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflectionlimit(mut self, val: f32) -> Self {
        self.params.insert(
            "reflectionlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflectionlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflectionlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractionlimit(mut self, val: f32) -> Self {
        self.params.insert(
            "refractionlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refractionlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractionlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumelimit(mut self, val: f32) -> Self {
        self.params.insert(
            "volumelimit".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumelimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumelimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssslimit(mut self, val: f32) -> Self {
        self.params.insert(
            "ssslimit".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ssslimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ssslimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorlimit(mut self, val: f32) -> Self {
        self.params.insert(
            "colorlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colorlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorlimitindirect(mut self, val: f32) -> Self {
        self.params.insert(
            "colorlimitindirect".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colorlimitindirect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorlimitindirect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumevblurscale(mut self, val: f32) -> Self {
        self.params.insert(
            "volumevblurscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumevblurscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumevblurscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_causticsroughnessclamp(mut self, val: f32) -> Self {
        self.params.insert(
            "causticsroughnessclamp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_causticsroughnessclamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "causticsroughnessclamp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raybias(mut self, val: f32) -> Self {
        self.params.insert(
            "raybias".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_raybias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raybias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offscreenquality(mut self, val: f32) -> Self {
        self.params.insert(
            "offscreenquality".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_offscreenquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offscreenquality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dicingqualityscale(mut self, val: f32) -> Self {
        self.params.insert(
            "dicingqualityscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dicingqualityscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dicingqualityscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pixelfiltersize(mut self, val: f32) -> Self {
        self.params.insert(
            "pixelfiltersize".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pixelfiltersize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelfiltersize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_utoe(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_utoe".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_utoe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_utoe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_ushoulder(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_ushoulder".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_ushoulder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_ushoulder".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_slope(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_slope".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_slope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_slope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_toe(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_toe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_toe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_shoulder(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_shoulder".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_shoulder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_shoulder".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_linear(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_linear".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_linear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_linear".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_linearangle(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_linearangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_linearangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_linearangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_toelength(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_toelength".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_toelength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_toelength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_shoulderlength(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_shoulderlength".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_shoulderlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_shoulderlength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_shoulderangle(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_shoulderangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_shoulderangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_shoulderangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pixelaspectratio(mut self, val: f32) -> Self {
        self.params.insert(
            "pixelAspectRatio".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pixelaspectratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelAspectRatio".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_dwa_compression(mut self, val: f32) -> Self {
        self.params.insert(
            "image_dwa_compression".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_image_dwa_compression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_dwa_compression".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmzbias(mut self, val: f32) -> Self {
        self.params.insert(
            "dcmzbias".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dcmzbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmzbias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_variance(mut self, val: f32) -> Self {
        self.params.insert(
            "oracle_variance".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oracle_variance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_variance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varianceaa_thresh(mut self, val: f32) -> Self {
        self.params.insert(
            "varianceaa_thresh".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_varianceaa_thresh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "varianceaa_thresh".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savefrequency(mut self, val: f32) -> Self {
        self.params.insert(
            "savefrequency".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_savefrequency_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savefrequency".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cacheratio(mut self, val: f32) -> Self {
        self.params.insert(
            "cacheratio".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cacheratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cacheratio".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_head_depthcue_z(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "head_depthcue_z".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_head_depthcue_z_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_depthcue_z".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_foffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "foffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_foffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_depthcue_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_depthcue_color".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_depthcue_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_depthcue_color".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_datawindowndc(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "dataWindowNDC".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_datawindowndc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataWindowNDC".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_samplesperpixel(mut self, val: i32) -> Self {
        self.params.insert(
            "samplesperpixel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_samplesperpixel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplesperpixel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathtracedsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "pathtracedsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathtracedsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathtracedsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_ao_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "head_ao_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_head_ao_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_ao_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varianceaa_minsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "varianceaa_minsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_varianceaa_minsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "varianceaa_minsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varianceaa_maxsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "varianceaa_maxsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_varianceaa_maxsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "varianceaa_maxsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_screendoorlimit(mut self, val: i32) -> Self {
        self.params.insert(
            "screendoorlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_screendoorlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "screendoorlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_russianroulette_cutoff(mut self, val: i32) -> Self {
        self.params.insert(
            "russianroulette_cutoff".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_russianroulette_cutoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "russianroulette_cutoff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geosamples(mut self, val: i32) -> Self {
        self.params.insert(
            "geosamples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_geosamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geosamples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xformsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "xformsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_xformsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instance_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "instance_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_instance_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instance_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpeaovlimit(mut self, val: i32) -> Self {
        self.params.insert(
            "lpeaovlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lpeaovlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpeaovlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cryptomatterank_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("cryptomatterank{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cryptomatterank_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("cryptomatterank{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmcompression(mut self, val: i32) -> Self {
        self.params.insert(
            "dcmcompression".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_dcmcompression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmcompression".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_minsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "oracle_minsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_oracle_minsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_minsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiding_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "guiding_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_guiding_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiding_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_progressivepasses(mut self, val: i32) -> Self {
        self.params.insert(
            "progressivepasses".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_progressivepasses_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "progressivepasses".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bucketsize(mut self, val: i32) -> Self {
        self.params.insert(
            "bucketsize".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bucketsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bucketsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecacheratio(mut self, val: i32) -> Self {
        self.params.insert(
            "usecacheratio".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_usecacheratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecacheratio".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachesize(mut self, val: i32) -> Self {
        self.params.insert(
            "cachesize".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachesize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_threads(mut self, val: i32) -> Self {
        self.params.insert(
            "threads".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_threads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "threads".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_snapshotinterval(mut self, val: i32) -> Self {
        self.params.insert(
            "snapshotinterval".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_snapshotinterval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "snapshotinterval".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timelimit(mut self, val: i32) -> Self {
        self.params.insert(
            "timelimit".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_timelimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timelimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verbosity(mut self, val: i32) -> Self {
        self.params.insert(
            "verbosity".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_verbosity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verbosity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tileindex(mut self, val: i32) -> Self {
        self.params.insert(
            "husk_tileindex".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_husk_tileindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tileindex".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_previewscale(mut self, val: DriverKarmaPreviewscale) -> Self {
        self.params.insert(
            "previewscale".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_previewscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "previewscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_res_override(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "res_override".to_string(),
            houdini_ramen_core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_res_override_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res_override".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolution(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "resolution".to_string(),
            houdini_ramen_core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_resolution_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolution".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tilecount(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "husk_tilecount".to_string(),
            houdini_ramen_core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_husk_tilecount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tilecount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: DriverKarmaTrange) -> Self {
        self.params.insert(
            "trange".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolutionmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "resolutionMenu".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_resolutionmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolutionMenu".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_sampling_mode(mut self, val: i32) -> Self {
        self.params.insert(
            "light_sampling_mode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_light_sampling_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_sampling_mode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mblur(mut self, val: DriverKarmaMblur) -> Self {
        self.params.insert(
            "mblur".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mblur".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curve_basis(mut self, val: i32) -> Self {
        self.params.insert(
            "curve_basis".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_curve_basis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curve_basis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmofsize(mut self, val: DriverKarmaDcmofsize) -> Self {
        self.params.insert(
            "dcmofsize".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dcmofsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmofsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_ociomode(mut self, val: i32) -> Self {
        self.params.insert(
            "oracle_ociomode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_oracle_ociomode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_ociomode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_tonemapcurve(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "tonemapcurve".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_tonemapcurve_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemapcurve".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_res_fraction(mut self, val: &str) -> Self {
        self.params.insert(
            "res_fraction".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_res_fraction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res_fraction".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picture(mut self, val: &str) -> Self {
        self.params.insert(
            "picture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_picture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_engine(mut self, val: &str) -> Self {
        self.params.insert(
            "engine".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_engine_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "engine".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vblur(mut self, val: &str) -> Self {
        self.params.insert(
            "vblur".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vblur".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blurstyle(mut self, val: &str) -> Self {
        self.params.insert(
            "blurstyle".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blurstyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blurstyle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instance_vblur(mut self, val: &str) -> Self {
        self.params.insert(
            "instance_vblur".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_instance_vblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instance_vblur".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_point_style(mut self, val: &str) -> Self {
        self.params.insert(
            "point_style".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_point_style_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "point_style".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curve_style(mut self, val: &str) -> Self {
        self.params.insert(
            "curve_style".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_curve_style_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curve_style".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cull_backface(mut self, val: &str) -> Self {
        self.params.insert(
            "cull_backface".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cull_backface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cull_backface".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dicingcamera(mut self, val: &str) -> Self {
        self.params.insert(
            "dicingcamera".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dicingcamera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dicingcamera".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_candobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "candobjects".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_candobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "candobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_objects(mut self, val: &str) -> Self {
        self.params.insert(
            "objects".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_objects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objects".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matte(mut self, val: &str) -> Self {
        self.params.insert(
            "matte".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matte_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matte".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_phantom(mut self, val: &str) -> Self {
        self.params.insert(
            "phantom".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_phantom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "phantom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_excludeobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "excludeobjects".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_excludeobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "excludeobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packedhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "packedhandling".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_packedhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "packedhandling".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sololights(mut self, val: &str) -> Self {
        self.params.insert(
            "sololights".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sololights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sololights".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_candlights(mut self, val: &str) -> Self {
        self.params.insert(
            "candlights".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_candlights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "candlights".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lights(mut self, val: &str) -> Self {
        self.params.insert(
            "lights".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lights".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_excludelights(mut self, val: &str) -> Self {
        self.params.insert(
            "excludelights".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_excludelights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "excludelights".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pixelfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "pixelfilter".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pixelfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelfilter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_omitlpes(mut self, val: &str) -> Self {
        self.params.insert(
            "omitlpes".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_omitlpes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "omitlpes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "outputcs".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputcs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_format_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("format{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_format_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("format{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_datatype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("dataType{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_datatype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dataType{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcename_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("sourceName{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcename_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("sourceName{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcetype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("sourceType{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcetype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("sourceType{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("filter{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("filter{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cryptomattesidecar_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("cryptomattesidecar{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cryptomattesidecar_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("cryptomattesidecar{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcs_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("outputcs{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputcs_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("outputcs{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoiser(mut self, val: &str) -> Self {
        self.params.insert(
            "denoiser".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_denoiser_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoiser".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoise_aovs(mut self, val: &str) -> Self {
        self.params.insert(
            "denoise_aovs".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_denoise_aovs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoise_aovs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap(mut self, val: &str) -> Self {
        self.params.insert(
            "tonemap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tonemap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_aovs(mut self, val: &str) -> Self {
        self.params.insert(
            "tonemap_aovs".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tonemap_aovs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_aovs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_planes_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ocio_planes{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocio_planes_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_planes{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_outputspace_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ocio_outputspace{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocio_outputspace_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_outputspace{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_inputspace_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ocio_inputspace{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocio_inputspace_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_inputspace{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_looks_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ocio_looks{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocio_looks_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_looks{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aspectratioconformpolicy(mut self, val: &str) -> Self {
        self.params.insert(
            "aspectRatioConformPolicy".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aspectratioconformpolicy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aspectRatioConformPolicy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_artist(mut self, val: &str) -> Self {
        self.params.insert(
            "image_artist".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_artist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_artist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_comment(mut self, val: &str) -> Self {
        self.params.insert(
            "image_comment".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_comment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_comment".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_hostname(mut self, val: &str) -> Self {
        self.params.insert(
            "image_hostname".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_hostname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_hostname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_exr_compression(mut self, val: &str) -> Self {
        self.params.insert(
            "image_exr_compression".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_exr_compression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_exr_compression".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmfilename(mut self, val: &str) -> Self {
        self.params.insert(
            "dcmfilename".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dcmfilename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmfilename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmvars(mut self, val: &str) -> Self {
        self.params.insert(
            "dcmvars".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dcmvars_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmvars".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_convergence_mode(mut self, val: &str) -> Self {
        self.params.insert(
            "convergence_mode".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_convergence_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convergence_mode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_plane(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle_plane".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_plane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_plane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_ociodisplay(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle_ociodisplay".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_ociodisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_ociodisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_ocioview(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle_ocioview".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_ocioview_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_ocioview".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_ociocolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle_ociocolorspace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_ociocolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_ociocolorspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_productname(mut self, val: &str) -> Self {
        self.params.insert(
            "productName".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_productname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "productName".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imagemode(mut self, val: &str) -> Self {
        self.params.insert(
            "imagemode".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_imagemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imagemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bucketorder(mut self, val: &str) -> Self {
        self.params.insert(
            "bucketorder".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bucketorder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bucketorder".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savetodirectory(mut self, val: &str) -> Self {
        self.params.insert(
            "savetodirectory".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_savetodirectory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savetodirectory".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_purpose(mut self, val: &str) -> Self {
        self.params.insert(
            "purpose".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_purpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "purpose".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_complexity(mut self, val: &str) -> Self {
        self.params.insert(
            "complexity".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_complexity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "complexity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolvercontext(mut self, val: &str) -> Self {
        self.params.insert(
            "resolvercontext".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resolvercontext_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolvercontext".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optionname{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_optionname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optiontype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optiontype{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_optiontype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optiontype{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionstrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optionstrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_optionstrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionstrvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windowsconsole(mut self, val: &str) -> Self {
        self.params.insert(
            "windowsconsole".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_windowsconsole_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "windowsconsole".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_usdtrace(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_usdtrace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_usdtrace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_usdtrace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_chromefile(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_chromefile".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_chromefile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_chromefile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_stdout(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_stdout".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_stdout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_stdout".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_stderr(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_stderr".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_stderr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_stderr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tilesuffix(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_tilesuffix".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_tilesuffix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tilesuffix".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportcomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "exportcomponents".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_exportcomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportcomponents".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffusecomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "diffusecomponents".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_diffusecomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffusecomponents".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractcomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "refractcomponents".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refractcomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractcomponents".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumecomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "volumecomponents".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_volumecomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumecomponents".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssscomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "ssscomponents".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ssscomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ssscomponents".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_previewchannels(mut self, val: &str) -> Self {
        self.params.insert(
            "previewchannels".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_previewchannels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "previewchannels".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mplayname(mut self, val: &str) -> Self {
        self.params.insert(
            "mplayname".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mplayname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mplayname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_override_camerares(mut self, val: bool) -> Self {
        self.params.insert(
            "override_camerares".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_override_camerares_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "override_camerares".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allframesatonce(mut self, val: bool) -> Self {
        self.params.insert(
            "allframesatonce".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allframesatonce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allframesatonce".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_force_headlight(mut self, val: bool) -> Self {
        self.params.insert(
            "force_headlight".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_force_headlight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "force_headlight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_addfog(mut self, val: bool) -> Self {
        self.params.insert(
            "head_addfog".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_head_addfog_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_addfog".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorlimitindlinked(mut self, val: bool) -> Self {
        self.params.insert(
            "colorlimitindlinked".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_colorlimitindlinked_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorlimitindlinked".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledof(mut self, val: bool) -> Self {
        self.params.insert(
            "enabledof".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablemblur(mut self, val: bool) -> Self {
        self.params.insert(
            "enablemblur".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablemblur".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_samplesfromstage(mut self, val: bool) -> Self {
        self.params.insert(
            "samplesfromstage".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_samplesfromstage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplesfromstage".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableimageblur(mut self, val: bool) -> Self {
        self.params.insert(
            "enableimageblur".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableimageblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableimageblur".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_causticsenable(mut self, val: bool) -> Self {
        self.params.insert(
            "causticsenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_causticsenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "causticsenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_autoraybias(mut self, val: bool) -> Self {
        self.params.insert(
            "autoraybias".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_autoraybias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "autoraybias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainmaxrough(mut self, val: bool) -> Self {
        self.params.insert(
            "constrainmaxrough".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constrainmaxrough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainmaxrough".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedicingcamera(mut self, val: bool) -> Self {
        self.params.insert(
            "usedicingcamera".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedicingcamera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedicingcamera".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doomitlpes(mut self, val: bool) -> Self {
        self.params.insert(
            "doomitlpes".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doomitlpes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doomitlpes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dooutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "dooutputcs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dooutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dooutputcs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beauty(mut self, val: bool) -> Self {
        self.params.insert(
            "beauty".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beauty_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beauty".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "beautyperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beautyperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "beautyunshadowed".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beautyunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyunshadowed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "beautyunshadowedperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beautyunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyunshadowedperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadow(mut self, val: bool) -> Self {
        self.params.insert(
            "shadow".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "shadowperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadowperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuse".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuse".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuse".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseunshadowed".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseunshadowed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseunshadowedperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseunshadowedperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseunshadowed".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseunshadowed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseunshadowedperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseunshadowedperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowed".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowedperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowedperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseshadow(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseshadow".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseshadow".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseshadowperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseshadowperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseshadowperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseshadowperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseshadow(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseshadow".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseshadow".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseshadowperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseshadowperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseshadowperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseshadowperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseshadow(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseshadow".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseshadow".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseshadowperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseshadowperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseshadowperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseshadowperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedglossyreflection(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedglossyreflection".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedglossyreflection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedglossyreflection".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedglossyreflectionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedglossyreflectionperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedglossyreflectionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedglossyreflectionperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directglossyreflection(mut self, val: bool) -> Self {
        self.params.insert(
            "directglossyreflection".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directglossyreflection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directglossyreflection".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directglossyreflectionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directglossyreflectionperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directglossyreflectionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directglossyreflectionperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectglossyreflection(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectglossyreflection".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectglossyreflection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectglossyreflection".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectglossyreflectionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectglossyreflectionperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectglossyreflectionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectglossyreflectionperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossytransmission(mut self, val: bool) -> Self {
        self.params.insert(
            "glossytransmission".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_glossytransmission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossytransmission".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossytransmissionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "glossytransmissionperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_glossytransmissionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossytransmissionperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coat(mut self, val: bool) -> Self {
        self.params.insert(
            "coat".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "coatperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coatperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedemission(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedemission".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedemission".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directemission(mut self, val: bool) -> Self {
        self.params.insert(
            "directemission".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directemission".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectemission(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectemission".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectemission".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visiblelights(mut self, val: bool) -> Self {
        self.params.insert(
            "visiblelights".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visiblelights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visiblelights".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visiblelightsperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "visiblelightsperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visiblelightsperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visiblelightsperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedvolume".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedvolume".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedvolumeperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedvolumeperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedvolumeperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedvolumeperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "directvolume".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directvolume".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directvolumeperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directvolumeperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directvolumeperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directvolumeperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectvolume".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectvolume".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectvolumeperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectvolumeperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectvolumeperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectvolumeperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss(mut self, val: bool) -> Self {
        self.params.insert(
            "sss".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sss_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sssperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "sssperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sssperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sssperlpe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_albedo(mut self, val: bool) -> Self {
        self.params.insert(
            "albedo".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_albedo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "albedo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ambientocclusion(mut self, val: bool) -> Self {
        self.params.insert(
            "ambientocclusion".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ambientocclusion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ambientocclusion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitp(mut self, val: bool) -> Self {
        self.params.insert(
            "hitP".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitP".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitp_camera(mut self, val: bool) -> Self {
        self.params.insert(
            "hitP_camera".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitp_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitP_camera".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpz(mut self, val: bool) -> Self {
        self.params.insert(
            "hitPz".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitpz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hituv(mut self, val: bool) -> Self {
        self.params.insert(
            "hituv".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hituv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hituv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitn(mut self, val: bool) -> Self {
        self.params.insert(
            "hitN".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitn_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitN".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitng(mut self, val: bool) -> Self {
        self.params.insert(
            "hitNg".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitng_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitn_facingratio(mut self, val: bool) -> Self {
        self.params.insert(
            "hitN_facingratio".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitn_facingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitN_facingratio".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitng_facingratio(mut self, val: bool) -> Self {
        self.params.insert(
            "hitNg_facingratio".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitng_facingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNg_facingratio".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motionvectors(mut self, val: bool) -> Self {
        self.params.insert(
            "motionvectors".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_motionvectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motionvectors".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cryptomatte_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("cryptomatte{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cryptomatte_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("cryptomatte{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dooutputcs_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("dooutputcs{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dooutputcs_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dooutputcs{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoise_usealbedo(mut self, val: bool) -> Self {
        self.params.insert(
            "denoise_usealbedo".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_denoise_usealbedo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoise_usealbedo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoise_usen(mut self, val: bool) -> Self {
        self.params.insert(
            "denoise_useN".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_denoise_usen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoise_useN".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("ocio_enable{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ocio_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_enable{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcm(mut self, val: bool) -> Self {
        self.params.insert(
            "dcm".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dcm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiding_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "guiding_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guiding_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiding_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcheckpoint(mut self, val: bool) -> Self {
        self.params.insert(
            "outputcheckpoint".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputcheckpoint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputcheckpoint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resume(mut self, val: bool) -> Self {
        self.params.insert(
            "resume".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resume".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domaxthreads(mut self, val: bool) -> Self {
        self.params.insert(
            "domaxthreads".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domaxthreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domaxthreads".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dosnapshot(mut self, val: bool) -> Self {
        self.params.insert(
            "dosnapshot".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosnapshot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dosnapshot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addtimelimit(mut self, val: bool) -> Self {
        self.params.insert(
            "addtimelimit".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addtimelimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addtimelimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_timelimitperimage(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_timelimitperimage".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_timelimitperimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_timelimitperimage".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_abortmissingtexture(mut self, val: bool) -> Self {
        self.params.insert(
            "abortmissingtexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_abortmissingtexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "abortmissingtexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_abortnogpudevice(mut self, val: bool) -> Self {
        self.params.insert(
            "abortnogpudevice".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_abortnogpudevice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "abortnogpudevice".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_runcommand(mut self, val: bool) -> Self {
        self.params.insert(
            "runcommand".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_runcommand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "runcommand".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strippostlayers(mut self, val: bool) -> Self {
        self.params.insert(
            "strippostlayers".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_strippostlayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strippostlayers".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soho_foreground(mut self, val: bool) -> Self {
        self.params.insert(
            "soho_foreground".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_soho_foreground_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soho_foreground".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_legacyexr(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_legacyexr".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_legacyexr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_legacyexr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_debug(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_debug".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_debug_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_debug".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_log(mut self, val: bool) -> Self {
        self.params.insert(
            "log".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_log_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "log".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_logappend(mut self, val: bool) -> Self {
        self.params.insert(
            "logappend".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_logappend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "logappend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tile(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_tile".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_tile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablemplaymonitor(mut self, val: bool) -> Self {
        self.params.insert(
            "enablemplaymonitor".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemplaymonitor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablemplaymonitor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DriverKarma {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "karma"
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
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait DriverKarmaInnerExt {
    fn lopnet(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rop_usdrender(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> DriverKarmaInnerExt for houdini_ramen_core::graph::InnerGraph<'a, DriverKarma> {
    fn lopnet(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("lopnet")
    }
    fn rop_usdrender(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rop_usdrender")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverKinefxFilmboxfbxanimationCliprangemode {
    UseClipinfoDetailAttribute = 0,
    RenderCurrentFrame = 1,
    RenderFrameRange = 2,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverKinefxFilmboxfbxanimationOutputunit {
    Centimeters = 0,
    Meters = 1,
}

#[derive(Debug, Clone)]
pub struct DriverKinefxFilmboxfbxanimation {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverKinefxFilmboxfbxanimation {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
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
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_jointdisplayscale(mut self, val: f32) -> Self {
        self.params.insert(
            "jointdisplayscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jointdisplayscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jointdisplayscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_cliprangemode(mut self, val: DriverKinefxFilmboxfbxanimationCliprangemode) -> Self {
        self.params.insert(
            "cliprangemode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cliprangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cliprangemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportpreset(mut self, val: i32) -> Self {
        self.params.insert(
            "exportpreset".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_exportpreset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportpreset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_axissystem(mut self, val: i32) -> Self {
        self.params.insert(
            "axissystem".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_axissystem_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axissystem".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputunit(mut self, val: DriverKinefxFilmboxfbxanimationOutputunit) -> Self {
        self.params.insert(
            "outputunit".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_outputunit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputunit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputfilepath(mut self, val: &str) -> Self {
        self.params.insert(
            "inputfilepath".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputfilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputfilepath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputfilepath(mut self, val: &str) -> Self {
        self.params.insert(
            "outputfilepath".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputfilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputfilepath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipname(mut self, val: &str) -> Self {
        self.params.insert(
            "clipname".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clipname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clipname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restposeattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "restposeattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restposeattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restposeattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_userestpose(mut self, val: bool) -> Self {
        self.params.insert(
            "userestpose".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userestpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "userestpose".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shiftstart(mut self, val: bool) -> Self {
        self.params.insert(
            "shiftstart".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shiftstart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shiftstart".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overriderate(mut self, val: bool) -> Self {
        self.params.insert(
            "overriderate".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overriderate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overriderate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savebinary(mut self, val: bool) -> Self {
        self.params.insert(
            "savebinary".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savebinary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savebinary".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportuserdefattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "exportuserdefattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exportuserdefattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportuserdefattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removeconstantanimcurves(mut self, val: bool) -> Self {
        self.params.insert(
            "removeconstantanimcurves".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removeconstantanimcurves_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "removeconstantanimcurves".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_convertaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "convertaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_convertaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convertaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_convertunits(mut self, val: bool) -> Self {
        self.params.insert(
            "convertunits".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_convertunits_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convertunits".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computesmoothinggroups(mut self, val: bool) -> Self {
        self.params.insert(
            "computesmoothinggroups".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computesmoothinggroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computesmoothinggroups".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removejointscaling(mut self, val: bool) -> Self {
        self.params.insert(
            "removejointscaling".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removejointscaling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "removejointscaling".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DriverKinefxFilmboxfbxanimation {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "kinefx::filmboxfbxanimation"
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
pub enum DriverKinefxFilmboxfbxcharacterCliprangemode {
    UseClipinfoDetailAttribute = 0,
    RenderCurrentFrame = 1,
    RenderFrameRange = 2,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverKinefxFilmboxfbxcharacterSkinmethod {
    Linear = 0,
    DualQuaternion = 1,
    BlendDualQuaternionAndLinear = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverKinefxFilmboxfbxcharacterOutputunit {
    Centimeters = 0,
    Meters = 1,
}

#[derive(Debug, Clone)]
pub struct DriverKinefxFilmboxfbxcharacter {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverKinefxFilmboxfbxcharacter {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
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
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_jointdisplayscale(mut self, val: f32) -> Self {
        self.params.insert(
            "jointdisplayscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jointdisplayscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jointdisplayscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_cliprangemode(mut self, val: DriverKinefxFilmboxfbxcharacterCliprangemode) -> Self {
        self.params.insert(
            "cliprangemode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cliprangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cliprangemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinmethod(mut self, val: DriverKinefxFilmboxfbxcharacterSkinmethod) -> Self {
        self.params.insert(
            "skinmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_skinmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skinmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportpreset(mut self, val: i32) -> Self {
        self.params.insert(
            "exportpreset".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_exportpreset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportpreset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_axissystem(mut self, val: i32) -> Self {
        self.params.insert(
            "axissystem".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_axissystem_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axissystem".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputunit(mut self, val: DriverKinefxFilmboxfbxcharacterOutputunit) -> Self {
        self.params.insert(
            "outputunit".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_outputunit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputunit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skingeosop(mut self, val: &str) -> Self {
        self.params.insert(
            "skingeosop".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skingeosop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skingeosop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_captureposesop(mut self, val: &str) -> Self {
        self.params.insert(
            "captureposesop".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_captureposesop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "captureposesop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animatedposesop(mut self, val: &str) -> Self {
        self.params.insert(
            "animatedposesop".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animatedposesop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animatedposesop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputfilepath(mut self, val: &str) -> Self {
        self.params.insert(
            "inputfilepath".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputfilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputfilepath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputfilepath(mut self, val: &str) -> Self {
        self.params.insert(
            "outputfilepath".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputfilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputfilepath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dqblendattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "dqblendattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dqblendattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dqblendattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipname(mut self, val: &str) -> Self {
        self.params.insert(
            "clipname".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clipname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clipname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restposeattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "restposeattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restposeattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restposeattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_userestpose(mut self, val: bool) -> Self {
        self.params.insert(
            "userestpose".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userestpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "userestpose".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shiftstart(mut self, val: bool) -> Self {
        self.params.insert(
            "shiftstart".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shiftstart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shiftstart".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overriderate(mut self, val: bool) -> Self {
        self.params.insert(
            "overriderate".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overriderate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overriderate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savebinary(mut self, val: bool) -> Self {
        self.params.insert(
            "savebinary".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savebinary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savebinary".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_embedmedia(mut self, val: bool) -> Self {
        self.params.insert(
            "embedmedia".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_embedmedia_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "embedmedia".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportuserdefattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "exportuserdefattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exportuserdefattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportuserdefattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removeconstantanimcurves(mut self, val: bool) -> Self {
        self.params.insert(
            "removeconstantanimcurves".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removeconstantanimcurves_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "removeconstantanimcurves".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_convertaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "convertaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_convertaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convertaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_convertunits(mut self, val: bool) -> Self {
        self.params.insert(
            "convertunits".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_convertunits_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convertunits".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computesmoothinggroups(mut self, val: bool) -> Self {
        self.params.insert(
            "computesmoothinggroups".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computesmoothinggroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computesmoothinggroups".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removejointscaling(mut self, val: bool) -> Self {
        self.params.insert(
            "removejointscaling".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removejointscaling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "removejointscaling".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DriverKinefxFilmboxfbxcharacter {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "kinefx::filmboxfbxcharacter"
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
pub enum DriverKinefxGltfcharacterCliprangemode {
    UseClipinfoDetailAttribute = 0,
    RenderCurrentFrame = 1,
    RenderFrameRange = 2,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverKinefxGltfcharacterExporttype {
    DetectFromFilename = 0,
    Gltf = 1,
    Glb = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverKinefxGltfcharacterTexturesource {
    ExportFromMaterial = 0,
    CopyOriginalTextures = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverKinefxGltfcharacterImageformat {
    OriginalFormatOrPng = 0,
    OriginalFormatOrJpeg = 1,
    Png = 2,
    Jpeg = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverKinefxGltfcharacterMaxresolution {
    NoLimit = 0,
    /// 256x256
    N256x256 = 1,
    /// 512x512
    N512x512 = 2,
    /// 1024x1024
    N1024x1024 = 3,
    /// 2048x2048
    N2048x2048 = 4,
    /// 4096x4096
    N4096x4096 = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverKinefxGltfcharacterMeshnamesource {
    NodeName = 0,
    Attribute = 1,
}

#[derive(Debug, Clone)]
pub struct DriverKinefxGltfcharacter {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverKinefxGltfcharacter {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
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
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_imagequality(mut self, val: i32) -> Self {
        self.params.insert(
            "imagequality".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_imagequality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imagequality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_cliprangemode(mut self, val: DriverKinefxGltfcharacterCliprangemode) -> Self {
        self.params.insert(
            "cliprangemode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cliprangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cliprangemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exporttype(mut self, val: DriverKinefxGltfcharacterExporttype) -> Self {
        self.params.insert(
            "exporttype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_exporttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exporttype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_texturesource(mut self, val: DriverKinefxGltfcharacterTexturesource) -> Self {
        self.params.insert(
            "texturesource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_texturesource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "texturesource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imageformat(mut self, val: DriverKinefxGltfcharacterImageformat) -> Self {
        self.params.insert(
            "imageformat".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_imageformat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imageformat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxresolution(mut self, val: DriverKinefxGltfcharacterMaxresolution) -> Self {
        self.params.insert(
            "maxresolution".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maxresolution_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxresolution".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_meshnamesource(mut self, val: DriverKinefxGltfcharacterMeshnamesource) -> Self {
        self.params.insert(
            "meshnamesource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_meshnamesource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "meshnamesource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skingeosop(mut self, val: &str) -> Self {
        self.params.insert(
            "skingeosop".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skingeosop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skingeosop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_captureposesop(mut self, val: &str) -> Self {
        self.params.insert(
            "captureposesop".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_captureposesop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "captureposesop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animatedposesop(mut self, val: &str) -> Self {
        self.params.insert(
            "animatedposesop".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animatedposesop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animatedposesop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filepath(mut self, val: &str) -> Self {
        self.params.insert(
            "filepath".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filepath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_additionaltexture_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("additionaltexture{}_name", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_additionaltexture_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("additionaltexture{}_name", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_additionaltexture_path_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("additionaltexture{}_path", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_additionaltexture_path_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("additionaltexture{}_path", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipname(mut self, val: &str) -> Self {
        self.params.insert(
            "clipname".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clipname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clipname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copyright(mut self, val: &str) -> Self {
        self.params.insert(
            "copyright".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_copyright_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copyright".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_meshnameattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "meshnameattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_meshnameattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "meshnameattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_poweroftwo(mut self, val: bool) -> Self {
        self.params.insert(
            "poweroftwo".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_poweroftwo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "poweroftwo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flipnormalmapy(mut self, val: bool) -> Self {
        self.params.insert(
            "flipnormalmapy".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flipnormalmapy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flipnormalmapy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "customattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportmaterials(mut self, val: bool) -> Self {
        self.params.insert(
            "exportmaterials".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exportmaterials_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportmaterials".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportmeshname(mut self, val: bool) -> Self {
        self.params.insert(
            "exportmeshname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exportmeshname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportmeshname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportjointnames(mut self, val: bool) -> Self {
        self.params.insert(
            "exportjointnames".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exportjointnames_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportjointnames".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DriverKinefxGltfcharacter {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "kinefx::gltfcharacter"
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
