#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmaTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
    RenderFrameRangeFromStage = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmaMblur {
    OnByDefault = 0,
    OffByDefault = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmaThreads {
    UseAllProcessors = 0,
    UseFourProcessors = 1,
    UseAllProcessorsButOne = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmaWindowsconsole {
    None = 0,
    Wait = 1,
    NoWait = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmaPreviewscale {
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
pub struct LopKarma {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopKarma {
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
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: "Additional Render Products"
    pub fn set_input_additional_render_products<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Additional Render Products" and specifies the output index of the target node.
    pub fn set_input_additional_render_products_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_additional_render_products_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Button parameters ---
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert(
            "execute".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderpreview(mut self) -> Self {
        self.params.insert(
            "renderpreview".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_executebackground(mut self) -> Self {
        self.params.insert(
            "executebackground".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_head_ao_distance(mut self, val: f32) -> Self {
        self.params.insert(
            "head_ao_distance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_ao_distance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_ao_distance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_depthcue_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "head_depthcue_alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_depthcue_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_depthcue_alpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffusequality(mut self, val: f32) -> Self {
        self.params.insert(
            "diffusequality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diffusequality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffusequality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflectquality(mut self, val: f32) -> Self {
        self.params.insert(
            "reflectquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflectquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflectquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractquality(mut self, val: f32) -> Self {
        self.params.insert(
            "refractquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refractquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumequality(mut self, val: f32) -> Self {
        self.params.insert(
            "volumequality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumequality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumequality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sssquality(mut self, val: f32) -> Self {
        self.params.insert(
            "sssquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sssquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sssquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_sampling_quality(mut self, val: f32) -> Self {
        self.params.insert(
            "light_sampling_quality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_light_sampling_quality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_sampling_quality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumesteprate(mut self, val: f32) -> Self {
        self.params.insert(
            "volumesteprate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumesteprate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumesteprate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuselimit(mut self, val: f32) -> Self {
        self.params.insert(
            "diffuselimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diffuselimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuselimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflectionlimit(mut self, val: f32) -> Self {
        self.params.insert(
            "reflectionlimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflectionlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflectionlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractionlimit(mut self, val: f32) -> Self {
        self.params.insert(
            "refractionlimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refractionlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractionlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumelimit(mut self, val: f32) -> Self {
        self.params.insert(
            "volumelimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumelimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumelimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssslimit(mut self, val: f32) -> Self {
        self.params.insert(
            "ssslimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ssslimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ssslimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorlimit(mut self, val: f32) -> Self {
        self.params.insert(
            "colorlimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colorlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorlimitindirect(mut self, val: f32) -> Self {
        self.params.insert(
            "colorlimitindirect".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colorlimitindirect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorlimitindirect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumevblurscale(mut self, val: f32) -> Self {
        self.params.insert(
            "volumevblurscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumevblurscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumevblurscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_causticsroughnessclamp(mut self, val: f32) -> Self {
        self.params.insert(
            "causticsroughnessclamp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_causticsroughnessclamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "causticsroughnessclamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raybias(mut self, val: f32) -> Self {
        self.params.insert(
            "raybias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_raybias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raybias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offscreenquality(mut self, val: f32) -> Self {
        self.params.insert(
            "offscreenquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_offscreenquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offscreenquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dicingqualityscale(mut self, val: f32) -> Self {
        self.params.insert(
            "dicingqualityscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dicingqualityscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dicingqualityscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pixelfiltersize(mut self, val: f32) -> Self {
        self.params.insert(
            "pixelfiltersize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pixelfiltersize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelfiltersize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pixelaspectratio(mut self, val: f32) -> Self {
        self.params.insert(
            "pixelAspectRatio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pixelaspectratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelAspectRatio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_dwa_compression(mut self, val: f32) -> Self {
        self.params.insert(
            "image_dwa_compression".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_image_dwa_compression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_dwa_compression".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varianceaa_thresh(mut self, val: f32) -> Self {
        self.params.insert(
            "varianceaa_thresh".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_varianceaa_thresh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "varianceaa_thresh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_variance(mut self, val: f32) -> Self {
        self.params.insert(
            "oracle_variance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oracle_variance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_variance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savefrequency(mut self, val: f32) -> Self {
        self.params.insert(
            "savefrequency".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_savefrequency_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savefrequency".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cacheratio(mut self, val: f32) -> Self {
        self.params.insert(
            "cacheratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cacheratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cacheratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionfloatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("optionfloatvalue{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_optionfloatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionfloatvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_head_depthcue_z(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "head_depthcue_z".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_head_depthcue_z_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_depthcue_z".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("f".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_foffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "foffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_foffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_depthcue_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_depthcue_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_depthcue_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_depthcue_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_datawindowndc(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "dataWindowNDC".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_datawindowndc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataWindowNDC".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_samplesperpixel(mut self, val: i32) -> Self {
        self.params.insert(
            "samplesperpixel".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_samplesperpixel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplesperpixel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathtracedsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "pathtracedsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathtracedsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathtracedsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_ao_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "head_ao_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_head_ao_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_ao_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varianceaa_minsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "varianceaa_minsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_varianceaa_minsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "varianceaa_minsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varianceaa_maxsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "varianceaa_maxsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_varianceaa_maxsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "varianceaa_maxsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_screendoorlimit(mut self, val: i32) -> Self {
        self.params.insert(
            "screendoorlimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_screendoorlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "screendoorlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_russianroulette_cutoff(mut self, val: i32) -> Self {
        self.params.insert(
            "russianroulette_cutoff".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_russianroulette_cutoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "russianroulette_cutoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xformsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "xformsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_xformsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geosamples(mut self, val: i32) -> Self {
        self.params.insert(
            "geosamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_geosamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geosamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instance_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "instance_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_instance_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instance_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpeaovlimit(mut self, val: i32) -> Self {
        self.params.insert(
            "lpeaovlimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lpeaovlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpeaovlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cryptomatterank_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("cryptomatterank{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cryptomatterank_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("cryptomatterank{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiding_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "guiding_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_guiding_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiding_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_minsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "oracle_minsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_oracle_minsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_minsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bucketsize(mut self, val: i32) -> Self {
        self.params.insert(
            "bucketsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bucketsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bucketsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecacheratio(mut self, val: i32) -> Self {
        self.params.insert(
            "usecacheratio".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_usecacheratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecacheratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachesize(mut self, val: i32) -> Self {
        self.params.insert(
            "cachesize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_threads(mut self, val: LopKarmaThreads) -> Self {
        self.params.insert(
            "threads".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_threads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timelimit(mut self, val: i32) -> Self {
        self.params.insert(
            "timelimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_timelimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timelimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_progressivepasses(mut self, val: i32) -> Self {
        self.params.insert(
            "progressivepasses".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_progressivepasses_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "progressivepasses".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verbose(mut self, val: i32) -> Self {
        self.params.insert(
            "verbose".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_verbose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verbose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_previewscale(mut self, val: LopKarmaPreviewscale) -> Self {
        self.params.insert(
            "previewscale".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_previewscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "previewscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_resolution(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "resolution".to_string(),
            crate::core::types::ParamValue::Int2(val),
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

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: LopKarmaTrange) -> Self {
        self.params.insert(
            "trange".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolutionmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "resolutionMenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_resolutionmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolutionMenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_sampling_mode(mut self, val: i32) -> Self {
        self.params.insert(
            "light_sampling_mode".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_light_sampling_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_sampling_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mblur(mut self, val: LopKarmaMblur) -> Self {
        self.params.insert(
            "mblur".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_curve_basis(mut self, val: i32) -> Self {
        self.params.insert(
            "curve_basis".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_curve_basis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curve_basis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_ociomode(mut self, val: i32) -> Self {
        self.params.insert(
            "oracle_ociomode".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_oracle_ociomode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_ociomode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windowsconsole(mut self, val: LopKarmaWindowsconsole) -> Self {
        self.params.insert(
            "windowsconsole".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_windowsconsole_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "windowsconsole".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picture(mut self, val: &str) -> Self {
        self.params.insert(
            "picture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_picture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_engine(mut self, val: &str) -> Self {
        self.params.insert(
            "engine".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_engine_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "engine".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vblur(mut self, val: &str) -> Self {
        self.params.insert(
            "vblur".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instance_vblur(mut self, val: &str) -> Self {
        self.params.insert(
            "instance_vblur".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_instance_vblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instance_vblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_point_style(mut self, val: &str) -> Self {
        self.params.insert(
            "point_style".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_point_style_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "point_style".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curve_style(mut self, val: &str) -> Self {
        self.params.insert(
            "curve_style".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_curve_style_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curve_style".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cull_backface(mut self, val: &str) -> Self {
        self.params.insert(
            "cull_backface".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cull_backface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cull_backface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dicingcamera(mut self, val: &str) -> Self {
        self.params.insert(
            "dicingcamera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dicingcamera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dicingcamera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pixelfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "pixelfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pixelfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_omitlpes(mut self, val: &str) -> Self {
        self.params.insert(
            "omitlpes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_omitlpes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "omitlpes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_format_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("format{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_format_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("format{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_datatype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("dataType{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_datatype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dataType{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcename_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("sourceName{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcename_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("sourceName{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcetype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("sourceType{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcetype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("sourceType{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("filter{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("filter{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cryptomattesidecar_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("cryptomattesidecar{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cryptomattesidecar_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("cryptomattesidecar{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoiser(mut self, val: &str) -> Self {
        self.params.insert(
            "denoiser".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_denoiser_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoiser".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoise_aovs(mut self, val: &str) -> Self {
        self.params.insert(
            "denoise_aovs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_denoise_aovs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoise_aovs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_planes_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ocio_planes{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocio_planes_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_planes{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_outputspace_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ocio_outputspace{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocio_outputspace_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_outputspace{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_inputspace_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ocio_inputspace{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocio_inputspace_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_inputspace{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_looks_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ocio_looks{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocio_looks_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_looks{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aspectratioconformpolicy(mut self, val: &str) -> Self {
        self.params.insert(
            "aspectRatioConformPolicy".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aspectratioconformpolicy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aspectRatioConformPolicy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_artist(mut self, val: &str) -> Self {
        self.params.insert(
            "image_artist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_artist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_artist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_comment(mut self, val: &str) -> Self {
        self.params.insert(
            "image_comment".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_comment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_comment".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_hostname(mut self, val: &str) -> Self {
        self.params.insert(
            "image_hostname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_hostname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_hostname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_exr_compression(mut self, val: &str) -> Self {
        self.params.insert(
            "image_exr_compression".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_exr_compression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_exr_compression".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmfilename(mut self, val: &str) -> Self {
        self.params.insert(
            "dcmfilename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dcmfilename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmfilename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmvars(mut self, val: &str) -> Self {
        self.params.insert(
            "dcmvars".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dcmvars_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmvars".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_convergence_mode(mut self, val: &str) -> Self {
        self.params.insert(
            "convergence_mode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_convergence_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convergence_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_plane(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle_plane".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_plane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_plane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_ociodisplay(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle_ociodisplay".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_ociodisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_ociodisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_ocioview(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle_ocioview".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_ocioview_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_ocioview".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_ociocolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle_ociocolorspace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_ociocolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_ociocolorspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_productname(mut self, val: &str) -> Self {
        self.params.insert(
            "productName".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_productname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "productName".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bucketorder(mut self, val: &str) -> Self {
        self.params.insert(
            "bucketorder".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bucketorder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bucketorder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savetodirectory(mut self, val: &str) -> Self {
        self.params.insert(
            "savetodirectory".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_savetodirectory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savetodirectory".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_purpose(mut self, val: &str) -> Self {
        self.params.insert(
            "purpose".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_purpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "purpose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_complexity(mut self, val: &str) -> Self {
        self.params.insert(
            "complexity".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_complexity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "complexity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolvercontext(mut self, val: &str) -> Self {
        self.params.insert(
            "resolvercontext".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resolvercontext_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolvercontext".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optionname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_optionname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optiontype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optiontype{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_optiontype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optiontype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionstrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optionstrvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_optionstrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionstrvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imagemode(mut self, val: &str) -> Self {
        self.params.insert(
            "imagemode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_imagemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imagemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vexprofiling(mut self, val: &str) -> Self {
        self.params.insert(
            "vexprofiling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vexprofiling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vexprofiling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usdtrace(mut self, val: &str) -> Self {
        self.params.insert(
            "usdtrace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_usdtrace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usdtrace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stdout(mut self, val: &str) -> Self {
        self.params.insert(
            "stdout".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stdout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stdout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stderr(mut self, val: &str) -> Self {
        self.params.insert(
            "stderr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stderr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stderr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportcomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "exportcomponents".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_exportcomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportcomponents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffusecomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "diffusecomponents".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_diffusecomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffusecomponents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractcomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "refractcomponents".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refractcomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractcomponents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumecomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "volumecomponents".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_volumecomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumecomponents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssscomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "ssscomponents".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ssscomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ssscomponents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_previewchannels(mut self, val: &str) -> Self {
        self.params.insert(
            "previewchannels".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_previewchannels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "previewchannels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mplayname(mut self, val: &str) -> Self {
        self.params.insert(
            "mplayname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mplayname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mplayname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_cachefileattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "cachefileattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cachefileattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachefileattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allframesatonce(mut self, val: bool) -> Self {
        self.params.insert(
            "allframesatonce".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allframesatonce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allframesatonce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_force_headlight(mut self, val: bool) -> Self {
        self.params.insert(
            "force_headlight".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_force_headlight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "force_headlight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_addfog(mut self, val: bool) -> Self {
        self.params.insert(
            "head_addfog".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_head_addfog_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_addfog".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorlimitindlinked(mut self, val: bool) -> Self {
        self.params.insert(
            "colorlimitindlinked".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_colorlimitindlinked_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorlimitindlinked".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledof(mut self, val: bool) -> Self {
        self.params.insert(
            "enabledof".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablemblur(mut self, val: bool) -> Self {
        self.params.insert(
            "enablemblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablemblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disableimageblur(mut self, val: bool) -> Self {
        self.params.insert(
            "disableimageblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_disableimageblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "disableimageblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_causticsenable(mut self, val: bool) -> Self {
        self.params.insert(
            "causticsenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_causticsenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "causticsenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainmaxrough(mut self, val: bool) -> Self {
        self.params.insert(
            "constrainmaxrough".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constrainmaxrough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainmaxrough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedicingcamera(mut self, val: bool) -> Self {
        self.params.insert(
            "usedicingcamera".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedicingcamera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedicingcamera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importsecondaryinputvars(mut self, val: bool) -> Self {
        self.params.insert(
            "importsecondaryinputvars".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importsecondaryinputvars_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importsecondaryinputvars".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importsecondaryproducts(mut self, val: bool) -> Self {
        self.params.insert(
            "importsecondaryproducts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importsecondaryproducts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importsecondaryproducts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doomitlpes(mut self, val: bool) -> Self {
        self.params.insert(
            "doomitlpes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doomitlpes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doomitlpes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beauty(mut self, val: bool) -> Self {
        self.params.insert(
            "beauty".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beauty_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beauty".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "beautyperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beautyperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "beautyunshadowed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beautyunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyunshadowed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "beautyunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beautyunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedglossyreflection(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedglossyreflection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedglossyreflection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedglossyreflection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedglossyreflectionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedglossyreflectionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directglossyreflection(mut self, val: bool) -> Self {
        self.params.insert(
            "directglossyreflection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directglossyreflection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directglossyreflection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directglossyreflectionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directglossyreflectionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectglossyreflection(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectglossyreflection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectglossyreflection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectglossyreflection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectglossyreflectionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectglossyreflectionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossytransmission(mut self, val: bool) -> Self {
        self.params.insert(
            "glossytransmission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_glossytransmission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossytransmission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossytransmissionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "glossytransmissionperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_glossytransmissionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossytransmissionperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coat(mut self, val: bool) -> Self {
        self.params.insert(
            "coat".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "coatperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coatperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedemission(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directemission(mut self, val: bool) -> Self {
        self.params.insert(
            "directemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectemission(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visiblelights(mut self, val: bool) -> Self {
        self.params.insert(
            "visiblelights".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visiblelights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visiblelights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visiblelightsperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "visiblelightsperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visiblelightsperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visiblelightsperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedvolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedvolumeperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedvolumeperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "directvolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directvolumeperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directvolumeperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectvolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectvolumeperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectvolumeperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss(mut self, val: bool) -> Self {
        self.params.insert(
            "sss".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sss_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sssperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "sssperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sssperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sssperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_albedo(mut self, val: bool) -> Self {
        self.params.insert(
            "albedo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_albedo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "albedo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitp(mut self, val: bool) -> Self {
        self.params.insert(
            "hitP".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitP".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpz(mut self, val: bool) -> Self {
        self.params.insert(
            "hitPz".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitpz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hituv(mut self, val: bool) -> Self {
        self.params.insert(
            "hituv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hituv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hituv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitn(mut self, val: bool) -> Self {
        self.params.insert(
            "hitN".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitn_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitN".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitng(mut self, val: bool) -> Self {
        self.params.insert(
            "hitNg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitng_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motionvectors(mut self, val: bool) -> Self {
        self.params.insert(
            "motionvectors".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_motionvectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motionvectors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cryptomatte_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("cryptomatte{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cryptomatte_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("cryptomatte{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoise_usealbedo(mut self, val: bool) -> Self {
        self.params.insert(
            "denoise_usealbedo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_denoise_usealbedo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoise_usealbedo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoise_usen(mut self, val: bool) -> Self {
        self.params.insert(
            "denoise_useN".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_denoise_usen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoise_useN".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("ocio_enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ocio_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcm(mut self, val: bool) -> Self {
        self.params.insert(
            "dcm".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dcm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcm".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiding_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "guiding_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guiding_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiding_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcheckpoint(mut self, val: bool) -> Self {
        self.params.insert(
            "outputcheckpoint".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputcheckpoint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputcheckpoint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resume(mut self, val: bool) -> Self {
        self.params.insert(
            "resume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addtimelimit(mut self, val: bool) -> Self {
        self.params.insert(
            "addtimelimit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addtimelimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addtimelimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_abortmissingtexture(mut self, val: bool) -> Self {
        self.params.insert(
            "abortmissingtexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_abortmissingtexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "abortmissingtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_runcommand(mut self, val: bool) -> Self {
        self.params.insert(
            "runcommand".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_runcommand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "runcommand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_striplayerbreaks(mut self, val: bool) -> Self {
        self.params.insert(
            "striplayerbreaks".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_striplayerbreaks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "striplayerbreaks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initsim(mut self, val: bool) -> Self {
        self.params.insert(
            "initsim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_initsim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initsim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reportnetwork(mut self, val: bool) -> Self {
        self.params.insert(
            "reportnetwork".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reportnetwork_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reportnetwork".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soho_foreground(mut self, val: bool) -> Self {
        self.params.insert(
            "soho_foreground".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_soho_foreground_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soho_foreground".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alfprogress(mut self, val: bool) -> Self {
        self.params.insert(
            "alfprogress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alfprogress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alfprogress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timestamps(mut self, val: bool) -> Self {
        self.params.insert(
            "timestamps".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timestamps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timestamps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_log(mut self, val: bool) -> Self {
        self.params.insert(
            "log".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_log_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "log".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_logappend(mut self, val: bool) -> Self {
        self.params.insert(
            "logappend".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_logappend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "logappend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tprerender(mut self, val: bool) -> Self {
        self.params.insert(
            "tprerender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpreframe(mut self, val: bool) -> Self {
        self.params.insert(
            "tpreframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpostframe(mut self, val: bool) -> Self {
        self.params.insert(
            "tpostframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpostrender(mut self, val: bool) -> Self {
        self.params.insert(
            "tpostrender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablemplaymonitor(mut self, val: bool) -> Self {
        self.params.insert(
            "enablemplaymonitor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemplaymonitor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablemplaymonitor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopKarma {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "karma"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub enum LopKarmacryptomatteRenderproductmode {
    AddToExistingRenderProduct = 0,
    CreateRenderProduct = 1,
    CreateRenderProductForEachId = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmacryptomatteKindcryptomatchby {
    PrimPath = 0,
    Assetinfo = 1,
}

#[derive(Debug, Clone)]
pub struct LopKarmacryptomatte {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopKarmacryptomatte {
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
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_xn_driverparametersopenexrdwa_compression_y2bkh(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__driverparametersOpenEXRdwa_compression_y2bkh".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_driverparametersopenexrdwa_compression_y2bkh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__driverparametersOpenEXRdwa_compression_y2bkh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_renderproductmode(mut self, val: LopKarmacryptomatteRenderproductmode) -> Self {
        self.params.insert(
            "renderproductmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_renderproductmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderproductmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primcryptomaxoverlap(mut self, val: i32) -> Self {
        self.params.insert(
            "primcryptomaxoverlap".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primcryptomaxoverlap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primcryptomaxoverlap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mtlcryptomaxoverlap(mut self, val: i32) -> Self {
        self.params.insert(
            "mtlcryptomaxoverlap".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mtlcryptomaxoverlap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mtlcryptomaxoverlap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kindcryptomatchby_inst(
        mut self,
        index1: usize,
        val: LopKarmacryptomatteKindcryptomatchby,
    ) -> Self {
        self.params.insert(
            format!("kindcryptomatchby{}", index1),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_kindcryptomatchby_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("kindcryptomatchby{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kindcryptomaxoverlap_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("kindcryptomaxoverlap{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_kindcryptomaxoverlap_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("kindcryptomaxoverlap{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primvarcryptomaxoverlap_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("primvarcryptomaxoverlap{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primvarcryptomaxoverlap_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("primvarcryptomaxoverlap{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_renderproducts(mut self, val: &str) -> Self {
        self.params.insert(
            "renderproducts".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renderproducts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderproducts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderproductpath(mut self, val: &str) -> Self {
        self.params.insert(
            "renderproductpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renderproductpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderproductpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rendersettings(mut self, val: &str) -> Self {
        self.params.insert(
            "rendersettings".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rendersettings_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rendersettings".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cryptopicture(mut self, val: &str) -> Self {
        self.params.insert(
            "cryptopicture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cryptopicture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cryptopicture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_driverparametersartist_wcbk(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__driverparametersartist_wcbk".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_driverparametersartist_wcbk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__driverparametersartist_wcbk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_driverparameterscomment_jebk(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__driverparameterscomment_jebk".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_driverparameterscomment_jebk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__driverparameterscomment_jebk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_driverparametershostname_5fbk(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__driverparametershostname_5fbk".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_driverparametershostname_5fbk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__driverparametershostname_5fbk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_driverparametersopenexrcompression_gwbkh(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__driverparametersOpenEXRcompression_gwbkh".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_driverparametersopenexrcompression_gwbkh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__driverparametersOpenEXRcompression_gwbkh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primcryptoname(mut self, val: &str) -> Self {
        self.params.insert(
            "primcryptoname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primcryptoname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primcryptoname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primcryptofile(mut self, val: &str) -> Self {
        self.params.insert(
            "primcryptofile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primcryptofile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primcryptofile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primcryptopixelfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "primcryptopixelfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primcryptopixelfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primcryptopixelfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primcryptosidecar(mut self, val: &str) -> Self {
        self.params.insert(
            "primcryptosidecar".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primcryptosidecar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primcryptosidecar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mtlcryptoname(mut self, val: &str) -> Self {
        self.params.insert(
            "mtlcryptoname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mtlcryptoname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mtlcryptoname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mtlcryptofile(mut self, val: &str) -> Self {
        self.params.insert(
            "mtlcryptofile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mtlcryptofile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mtlcryptofile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mtlcryptopixelfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "mtlcryptopixelfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mtlcryptopixelfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mtlcryptopixelfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mtlcryptosidecar(mut self, val: &str) -> Self {
        self.params.insert(
            "mtlcryptosidecar".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mtlcryptosidecar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mtlcryptosidecar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kindcrypto_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("kindcrypto{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kindcrypto_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("kindcrypto{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kindcryptoname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("kindcryptoname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kindcryptoname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("kindcryptoname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kindcryptofile_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("kindcryptofile{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kindcryptofile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("kindcryptofile{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kindcryptopixelfilter_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("kindcryptopixelfilter{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kindcryptopixelfilter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("kindcryptopixelfilter{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kindcryptosidecar_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("kindcryptosidecar{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kindcryptosidecar_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("kindcryptosidecar{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primvarcrypto_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("primvarcrypto{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primvarcrypto_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("primvarcrypto{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primvarcryptoname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("primvarcryptoname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primvarcryptoname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("primvarcryptoname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primvarcryptofile_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("primvarcryptofile{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primvarcryptofile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("primvarcryptofile{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primvarcryptopixelfilter_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("primvarcryptopixelfilter{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primvarcryptopixelfilter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("primvarcryptopixelfilter{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primvarcryptosidecar_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("primvarcryptosidecar{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primvarcryptosidecar_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("primvarcryptosidecar{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_doprimcrypto(mut self, val: bool) -> Self {
        self.params.insert(
            "doprimcrypto".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doprimcrypto_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doprimcrypto".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primcryptomultisampled(mut self, val: bool) -> Self {
        self.params.insert(
            "primcryptomultisampled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_primcryptomultisampled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primcryptomultisampled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domtlcrypto(mut self, val: bool) -> Self {
        self.params.insert(
            "domtlcrypto".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domtlcrypto_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domtlcrypto".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mtlcryptomultisampled(mut self, val: bool) -> Self {
        self.params.insert(
            "mtlcryptomultisampled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mtlcryptomultisampled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mtlcryptomultisampled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dokindcrypto(mut self, val: bool) -> Self {
        self.params.insert(
            "dokindcrypto".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dokindcrypto_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dokindcrypto".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kindcryptostrict_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("kindcryptostrict{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_kindcryptostrict_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("kindcryptostrict{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kindcryptomultisampled_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("kindcryptomultisampled{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_kindcryptomultisampled_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("kindcryptomultisampled{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doprimvarcrypto(mut self, val: bool) -> Self {
        self.params.insert(
            "doprimvarcrypto".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doprimvarcrypto_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doprimvarcrypto".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primvarcryptomultisampled_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("primvarcryptomultisampled{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_primvarcryptomultisampled_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("primvarcryptomultisampled{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopKarmacryptomatte {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "karmacryptomatte"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub enum LopKarmafogboxShape {
    Box = 0,
    Sphere = 1,
    Tube = 2,
    Cone = 3,
    Capsule = 4,
    Torus = 5,
    Custom = 6,
}

#[derive(Debug, Clone)]
pub struct LopKarmafogbox {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopKarmafogbox {
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
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadow(mut self, val: f32) -> Self {
        self.params.insert(
            "shadow".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_phase(mut self, val: f32) -> Self {
        self.params.insert(
            "phase".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_phase_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "phase".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noisescale(mut self, val: f32) -> Self {
        self.params.insert(
            "noisescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_noisescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noisescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitude(mut self, val: f32) -> Self {
        self.params.insert(
            "amplitude".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amplitude_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amplitude".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lacunarity(mut self, val: f32) -> Self {
        self.params.insert(
            "lacunarity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lacunarity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lacunarity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_atten(mut self, val: f32) -> Self {
        self.params.insert(
            "atten".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_atten_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "atten".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_min(mut self, val: f32) -> Self {
        self.params.insert(
            "min".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "min".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_max(mut self, val: f32) -> Self {
        self.params.insert(
            "max".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_max_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "max".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

    // --- Float3 parameters ---
    pub fn with_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_freq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "freq".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_freq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "freq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
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

    // --- Int parameters ---
    pub fn with_octaves(mut self, val: i32) -> Self {
        self.params.insert(
            "octaves".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_octaves_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "octaves".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_shape(mut self, val: LopKarmafogboxShape) -> Self {
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

    // --- String parameters ---
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_purpose(mut self, val: &str) -> Self {
        self.params.insert(
            "purpose".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_purpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "purpose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customsop(mut self, val: &str) -> Self {
        self.params.insert(
            "customsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customsop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_addnoise(mut self, val: bool) -> Self {
        self.params.insert(
            "addnoise".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addnoise_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addnoise".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopKarmafogbox {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "karmafogbox"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub struct LopKarmaocean {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopKarmaocean {
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
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_time(mut self, val: f32) -> Self {
        self.params.insert(
            "time".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_time_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "time".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dicingquality(mut self, val: f32) -> Self {
        self.params.insert(
            "dicingquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dicingquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dicingquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_waveheightmax(mut self, val: f32) -> Self {
        self.params.insert(
            "waveheightmax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_waveheightmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "waveheightmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiordensity(mut self, val: f32) -> Self {
        self.params.insert(
            "interiordensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_interiordensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interiordensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_vm_volumesamples(mut self, val: i32) -> Self {
        self.params.insert(
            "vm_volumesamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vm_volumesamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_volumesamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spectrumfile(mut self, val: &str) -> Self {
        self.params.insert(
            "spectrumfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_spectrumfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spectrumfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskfile(mut self, val: &str) -> Self {
        self.params.insert(
            "maskfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maskfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_foamfile(mut self, val: &str) -> Self {
        self.params.insert(
            "foamfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_foamfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foamfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_proxygeo(mut self, val: &str) -> Self {
        self.params.insert(
            "proxygeo".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_proxygeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "proxygeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rendergeo(mut self, val: &str) -> Self {
        self.params.insert(
            "rendergeo".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rendergeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rendergeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacematerial(mut self, val: &str) -> Self {
        self.params.insert(
            "surfacematerial".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_surfacematerial_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfacematerial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiorgeo(mut self, val: &str) -> Self {
        self.params.insert(
            "interiorgeo".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_interiorgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interiorgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiormaterial(mut self, val: &str) -> Self {
        self.params.insert(
            "interiormaterial".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_interiormaterial_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interiormaterial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablefoam(mut self, val: bool) -> Self {
        self.params.insert(
            "enablefoam".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefoam_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefoam".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableproxy(mut self, val: bool) -> Self {
        self.params.insert(
            "enableproxy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableproxy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableproxy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputproxy(mut self, val: bool) -> Self {
        self.params.insert(
            "outputproxy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputproxy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputproxy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableinterior(mut self, val: bool) -> Self {
        self.params.insert(
            "enableinterior".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableinterior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableinterior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopKarmaocean {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "karmaocean"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("ocean_geometry")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmaphysicalskySkymode {
    DomeLight = 0,
    Atmospheric = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmaphysicalskySetUsing {
    /// Azimuth/Altitude
    AzimuthAltitude = 0,
    /// Location/Date/Time
    LocationDateTime = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmaphysicalskyGeoMonth {
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
pub enum LopKarmaphysicalskyGeoHrSpec {
    Am = 0,
    Pm = 1,
    /// 24hr
    N24hr = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmaphysicalskyGeoTimeZone {
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
pub enum LopKarmaphysicalskySunRenderlightgeo {
    ForceDisable = 0,
    ForceEnable = 1,
    UseGlobalControl = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmaphysicalskySkyRenderlightgeo {
    ForceDisable = 0,
    ForceEnable = 1,
    UseGlobalControl = 2,
}

#[derive(Debug, Clone)]
pub struct LopKarmaphysicalsky {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopKarmaphysicalsky {
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
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exposure(mut self, val: f32) -> Self {
        self.params.insert(
            "exposure".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_exposure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exposure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angle(mut self, val: f32) -> Self {
        self.params.insert(
            "angle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solar_altitude(mut self, val: f32) -> Self {
        self.params.insert(
            "solar_altitude".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solar_altitude_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solar_altitude".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solar_azimuth(mut self, val: f32) -> Self {
        self.params.insert(
            "solar_azimuth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solar_azimuth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solar_azimuth".to_string(),
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
    pub fn with_sun_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "sun_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sun_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_exposure(mut self, val: f32) -> Self {
        self.params.insert(
            "sun_exposure".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sun_exposure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_exposure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_diffuse(mut self, val: f32) -> Self {
        self.params.insert(
            "sun_diffuse".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sun_diffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_diffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_specular(mut self, val: f32) -> Self {
        self.params.insert(
            "sun_specular".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sun_specular_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_specular".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_misbias(mut self, val: f32) -> Self {
        self.params.insert(
            "sun_misbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sun_misbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_misbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_samplingquality(mut self, val: f32) -> Self {
        self.params.insert(
            "sun_samplingquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sun_samplingquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_samplingquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_horizon_blur(mut self, val: f32) -> Self {
        self.params.insert(
            "horizon_blur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_horizon_blur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "horizon_blur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "sky_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sky_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_exposure(mut self, val: f32) -> Self {
        self.params.insert(
            "sky_exposure".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sky_exposure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_exposure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_diffuse(mut self, val: f32) -> Self {
        self.params.insert(
            "sky_diffuse".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sky_diffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_diffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_specular(mut self, val: f32) -> Self {
        self.params.insert(
            "sky_specular".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sky_specular_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_specular".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_misbias(mut self, val: f32) -> Self {
        self.params.insert(
            "sky_misbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sky_misbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_misbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_samplingquality(mut self, val: f32) -> Self {
        self.params.insert(
            "sky_samplingquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sky_samplingquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_samplingquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_portalmisbias(mut self, val: f32) -> Self {
        self.params.insert(
            "sky_portalmisbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sky_portalmisbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_portalmisbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rayleighscale(mut self, val: f32) -> Self {
        self.params.insert(
            "rayleighscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rayleighscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rayleighscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rayleighdist(mut self, val: f32) -> Self {
        self.params.insert(
            "rayleighdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rayleighdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rayleighdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_miescatterscale(mut self, val: f32) -> Self {
        self.params.insert(
            "miescatterscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_miescatterscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "miescatterscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mieabsorpscale(mut self, val: f32) -> Self {
        self.params.insert(
            "mieabsorpscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mieabsorpscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mieabsorpscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_miedist(mut self, val: f32) -> Self {
        self.params.insert(
            "miedist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_miedist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "miedist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmiephase_61a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarsmiephase_61a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarsmiephase_61a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsmiephase_61a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extrascale(mut self, val: f32) -> Self {
        self.params.insert(
            "extrascale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_extrascale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extrascale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extraaltitude(mut self, val: f32) -> Self {
        self.params.insert(
            "extraaltitude".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_extraaltitude_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extraaltitude".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extrawidth(mut self, val: f32) -> Self {
        self.params.insert(
            "extrawidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_extrawidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extrawidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sealevel(mut self, val: f32) -> Self {
        self.params.insert(
            "sealevel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sealevel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sealevel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unitscale(mut self, val: f32) -> Self {
        self.params.insert(
            "unitscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unitscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unitscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stepsizemin(mut self, val: f32) -> Self {
        self.params.insert(
            "stepsizemin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stepsizemin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stepsizemin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stepsizemax(mut self, val: f32) -> Self {
        self.params.insert(
            "stepsizemax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stepsizemax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stepsizemax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundoffset_p8a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarsgroundoffset_p8a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarsgroundoffset_p8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundoffset_p8a".to_string(),
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
    pub fn with_xn_primvarslutdim_xya(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "xn__primvarslutdim_xya".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_xn_primvarslutdim_xya_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarslutdim_xya".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sun_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sun_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sun_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ground_albedo(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ground_albedo".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ground_albedo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ground_albedo".to_string(),
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
    pub fn with_sky_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sky_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sky_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rayleighcol(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rayleighcol".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rayleighcol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rayleighcol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_miescattercol(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "miescattercol".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_miescattercol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "miescattercol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mieabsorpcol(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "mieabsorpcol".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_mieabsorpcol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mieabsorpcol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extraabsorpcol(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "extraabsorpcol".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_extraabsorpcol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extraabsorpcol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundalbedo_p8a(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__primvarsgroundalbedo_p8a".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_primvarsgroundalbedo_p8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundalbedo_p8a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_planetcenter(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "planetcenter".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_planetcenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planetcenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_skymode(mut self, val: LopKarmaphysicalskySkymode) -> Self {
        self.params.insert(
            "skymode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_skymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_set_using(mut self, val: LopKarmaphysicalskySetUsing) -> Self {
        self.params.insert(
            "set_using".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_set_using_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "set_using".to_string(),
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
    pub fn with_sun_renderlightgeo(mut self, val: LopKarmaphysicalskySunRenderlightgeo) -> Self {
        self.params.insert(
            "sun_renderlightgeo".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_sun_renderlightgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_renderlightgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_renderlightgeo(mut self, val: LopKarmaphysicalskySkyRenderlightgeo) -> Self {
        self.params.insert(
            "sky_renderlightgeo".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_sky_renderlightgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_renderlightgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarskarmaobjectvolumelimit_zpbfg(mut self, val: i32) -> Self {
        self.params.insert(
            "xn__primvarskarmaobjectvolumelimit_zpbfg".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_xn_primvarskarmaobjectvolumelimit_zpbfg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarskarmaobjectvolumelimit_zpbfg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_geo_month(mut self, val: LopKarmaphysicalskyGeoMonth) -> Self {
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
    pub fn with_geo_hr_spec(mut self, val: LopKarmaphysicalskyGeoHrSpec) -> Self {
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
    pub fn with_geo_time_zone(mut self, val: LopKarmaphysicalskyGeoTimeZone) -> Self {
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

    // --- String parameters ---
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intensity_control(mut self, val: &str) -> Self {
        self.params.insert(
            "intensity_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_intensity_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "intensity_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exposure_control(mut self, val: &str) -> Self {
        self.params.insert(
            "exposure_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_exposure_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exposure_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angle_control(mut self, val: &str) -> Self {
        self.params.insert(
            "angle_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_angle_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angle_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solar_altitude_control(mut self, val: &str) -> Self {
        self.params.insert(
            "solar_altitude_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_solar_altitude_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solar_altitude_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solar_azimuth_control(mut self, val: &str) -> Self {
        self.params.insert(
            "solar_azimuth_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_solar_azimuth_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solar_azimuth_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_intensity_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sun_intensity_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sun_intensity_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_intensity_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_exposure_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sun_exposure_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sun_exposure_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_exposure_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_color_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sun_color_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sun_color_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_color_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_diffuse_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sun_diffuse_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sun_diffuse_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_diffuse_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_specular_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sun_specular_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sun_specular_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_specular_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_renderlightgeo_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sun_renderlightgeo_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sun_renderlightgeo_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_renderlightgeo_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_misbias_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sun_misbias_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sun_misbias_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_misbias_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_samplingmode_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sun_samplingmode_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sun_samplingmode_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_samplingmode_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_samplingmode(mut self, val: &str) -> Self {
        self.params.insert(
            "sun_samplingmode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sun_samplingmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_samplingmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_samplingquality_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sun_samplingquality_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sun_samplingquality_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_samplingquality_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_contribs_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sun_contribs_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sun_contribs_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_contribs_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_contribs(mut self, val: &str) -> Self {
        self.params.insert(
            "sun_contribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sun_contribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_contribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_lpetag_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sun_lpetag_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sun_lpetag_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_lpetag_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sun_lpetag(mut self, val: &str) -> Self {
        self.params.insert(
            "sun_lpetag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sun_lpetag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sun_lpetag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_turbidity_control(mut self, val: &str) -> Self {
        self.params.insert(
            "turbidity_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_turbidity_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "turbidity_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ground_albedo_control(mut self, val: &str) -> Self {
        self.params.insert(
            "ground_albedo_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ground_albedo_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ground_albedo_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ground_color_control(mut self, val: &str) -> Self {
        self.params.insert(
            "ground_color_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ground_color_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ground_color_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_horizon_blur_control(mut self, val: &str) -> Self {
        self.params.insert(
            "horizon_blur_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_horizon_blur_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "horizon_blur_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_intensity_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sky_intensity_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sky_intensity_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_intensity_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_exposure_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sky_exposure_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sky_exposure_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_exposure_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_color_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sky_color_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sky_color_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_color_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_diffuse_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sky_diffuse_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sky_diffuse_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_diffuse_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_specular_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sky_specular_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sky_specular_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_specular_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_renderlightgeo_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sky_renderlightgeo_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sky_renderlightgeo_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_renderlightgeo_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_misbias_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sky_misbias_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sky_misbias_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_misbias_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_samplingmode_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sky_samplingmode_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sky_samplingmode_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_samplingmode_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_samplingmode(mut self, val: &str) -> Self {
        self.params.insert(
            "sky_samplingmode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sky_samplingmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_samplingmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_samplingquality_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sky_samplingquality_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sky_samplingquality_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_samplingquality_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_portalmisbias_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sky_portalmisbias_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sky_portalmisbias_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_portalmisbias_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_contribs_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sky_contribs_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sky_contribs_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_contribs_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_contribs(mut self, val: &str) -> Self {
        self.params.insert(
            "sky_contribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sky_contribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_contribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_lpetag_control(mut self, val: &str) -> Self {
        self.params.insert(
            "sky_lpetag_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sky_lpetag_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_lpetag_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sky_lpetag(mut self, val: &str) -> Self {
        self.params.insert(
            "sky_lpetag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sky_lpetag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sky_lpetag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rayleighcoeff_control(mut self, val: &str) -> Self {
        self.params.insert(
            "rayleighcoeff_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rayleighcoeff_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rayleighcoeff_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsrayleighdist_control_qmb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsrayleighdist_control_qmb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsrayleighdist_control_qmb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsrayleighdist_control_qmb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmiecoeff_control_7fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsmiecoeff_control_7fb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmiecoeff_control_7fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsmiecoeff_control_7fb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmieabsorpcoeff_control_zpb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsmieabsorpcoeff_control_zpb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmieabsorpcoeff_control_zpb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsmieabsorpcoeff_control_zpb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmiedist_control_leb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsmiedist_control_leb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmiedist_control_leb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsmiedist_control_leb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmiephase_control_7fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsmiephase_control_7fb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmiephase_control_7fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsmiephase_control_7fb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsextraabsorption_control_mrb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsextraabsorption_control_mrb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsextraabsorption_control_mrb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsextraabsorption_control_mrb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsextraaltitude_control_dob(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsextraaltitude_control_dob".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsextraaltitude_control_dob_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsextraaltitude_control_dob".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsextrawidth_control_hjb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsextrawidth_control_hjb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsextrawidth_control_hjb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsextrawidth_control_hjb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundalbedo_control_qmb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundalbedo_control_qmb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundalbedo_control_qmb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundalbedo_control_qmb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundvisibility_control_8sb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundvisibility_control_8sb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundvisibility_control_8sb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundvisibility_control_8sb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundvisibility_7fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundvisibility_7fb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundvisibility_7fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundvisibility_7fb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsplanetcenter_control_qmb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsplanetcenter_control_qmb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsplanetcenter_control_qmb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsplanetcenter_control_qmb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarssealevel_control_7fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarssealevel_control_7fb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarssealevel_control_7fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarssealevel_control_7fb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarskarmaobjectlpetag_control_vubfg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarskarmaobjectlpetag_control_vubfg".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarskarmaobjectlpetag_control_vubfg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarskarmaobjectlpetag_control_vubfg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarskarmaobjectlpetag_uhbfg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarskarmaobjectlpetag_uhbfg".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarskarmaobjectlpetag_uhbfg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarskarmaobjectlpetag_uhbfg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsstepsizemin_control_3kb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsstepsizemin_control_3kb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsstepsizemin_control_3kb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsstepsizemin_control_3kb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsstepsizemax_control_3kb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsstepsizemax_control_3kb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsstepsizemax_control_3kb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsstepsizemax_control_3kb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarskarmaobjectvolumelimit_control_02bfg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarskarmaobjectvolumelimit_control_02bfg".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarskarmaobjectvolumelimit_control_02bfg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarskarmaobjectvolumelimit_control_02bfg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsociofrom_control_7fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsociofrom_control_7fb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsociofrom_control_7fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsociofrom_control_7fb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsociofrom_61a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsociofrom_61a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsociofrom_61a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsociofrom_61a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarslutdim_control_ycb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarslutdim_control_ycb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarslutdim_control_ycb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarslutdim_control_ycb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundoffset_control_qmb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundoffset_control_qmb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundoffset_control_qmb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundoffset_control_qmb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundreceiveshadow_control_4xb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundreceiveshadow_control_4xb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundreceiveshadow_control_4xb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundreceiveshadow_control_4xb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsrenderbackside_control_zpb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsrenderbackside_control_zpb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsrenderbackside_control_zpb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsrenderbackside_control_zpb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarssolidalphainipr_control_mrb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarssolidalphainipr_control_mrb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarssolidalphainipr_control_mrb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarssolidalphainipr_control_mrb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skyprim(mut self, val: &str) -> Self {
        self.params.insert(
            "skyprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skyprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skyprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sunprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sunprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sunprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sunprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_atmoprim(mut self, val: &str) -> Self {
        self.params.insert(
            "atmoprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_atmoprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "atmoprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lightmap(mut self, val: &str) -> Self {
        self.params.insert(
            "lightmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lightmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lightmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upaxis(mut self, val: &str) -> Self {
        self.params.insert(
            "upaxis".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_upaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_renderlightgeo(mut self, val: bool) -> Self {
        self.params.insert(
            "renderlightgeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_renderlightgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderlightgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_enablesun(mut self, val: bool) -> Self {
        self.params.insert(
            "enablesun".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesun_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablesun".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computegroundcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "computegroundcolor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computegroundcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computegroundcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesky(mut self, val: bool) -> Self {
        self.params.insert(
            "enablesky".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesky_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablesky".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundreceiveshadow_3kb(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__primvarsgroundreceiveshadow_3kb".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_primvarsgroundreceiveshadow_3kb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundreceiveshadow_3kb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsrenderbackside_ycb(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__primvarsrenderbackside_ycb".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_primvarsrenderbackside_ycb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsrenderbackside_ycb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarssolidalphainipr_leb(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__primvarssolidalphainipr_leb".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_primvarssolidalphainipr_leb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarssolidalphainipr_leb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopKarmaphysicalsky {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "karmaphysicalsky"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub enum LopKarmaproceduralType {
    Sphere = 0,
    File = 1,
    VdbIsoSurface = 2,
}

#[derive(Debug, Clone)]
pub struct LopKarmaprocedural {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopKarmaprocedural {
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
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_radius_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("radius{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radius_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("radius{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_color_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("color{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_color_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("color{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_type_inst(mut self, index1: usize, val: LopKarmaproceduralType) -> Self {
        self.params.insert(
            format!("type{}", index1),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("type{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("group{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("group{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_radiussnippet_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("radiussnippet{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_radiussnippet_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("radiussnippet{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorsnippet_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("colorsnippet{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_colorsnippet_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("colorsnippet{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filename_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("filename{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filename_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("filename{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vdbfile_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("vdbfile{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vdbfile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("vdbfile{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useradius_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("useradius{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useradius_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("useradius{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useradiussnippet_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("useradiussnippet{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useradiussnippet_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("useradiussnippet{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecolor_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("usecolor{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecolor_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("usecolor{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecolorsnippet_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("usecolorsnippet{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecolorsnippet_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("usecolorsnippet{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopKarmaprocedural {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "karmaprocedural"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("sopnet/subnet")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait LopKarmaproceduralInnerExt {
    fn add1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> LopKarmaproceduralInnerExt for crate::core::graph::InnerGraph<'a, LopKarmaprocedural> {
    fn add1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("sopnet/subnet/add1")
    }
}

#[derive(Debug, Clone)]
pub struct LopKarmarenderproducts {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopKarmarenderproducts {
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
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_pixelaspectratio(mut self, val: f32) -> Self {
        self.params.insert(
            "pixelAspectRatio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pixelaspectratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelAspectRatio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_datawindowndc(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "dataWindowNDC".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_datawindowndc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataWindowNDC".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_resolution(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "resolution".to_string(),
            crate::core::types::ParamValue::Int2(val),
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

    // --- Menu parameters ---
    pub fn with_resolutionmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "resolutionMenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_resolutionmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolutionMenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_parentprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orderedvars(mut self, val: &str) -> Self {
        self.params.insert(
            "orderedVars".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_orderedvars_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orderedVars".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aspectratioconformpolicy(mut self, val: &str) -> Self {
        self.params.insert(
            "aspectRatioConformPolicy".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aspectratioconformpolicy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aspectRatioConformPolicy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "outputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("primname_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("primname_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_productname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("productName_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_productname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("productName_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orderedvars_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("orderedVars_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_orderedvars_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("orderedVars_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_camera_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("camera_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_camera_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("camera_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dooutputcs_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("dooutputcs_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dooutputcs_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dooutputcs_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcs_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("outputcs_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputcs_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("outputcs_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_instantaneousshutter(mut self, val: bool) -> Self {
        self.params.insert(
            "instantaneousShutter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_instantaneousshutter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instantaneousShutter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dooutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "dooutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dooutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dooutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enable_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enable_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doorderedvars_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("doorderedVars_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doorderedvars_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("doorderedVars_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docamera_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("docamera_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docamera_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("docamera_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopKarmarenderproducts {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "karmarenderproducts"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub enum LopKarmarenderpropertiesQuicksetup {
    /// Quick Setups ↓
    QuickSetups = 0,
    OptimizationAovs = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmarenderpropertiesMblur {
    OnByDefault = 0,
    OffByDefault = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmarenderpropertiesDcmofsize {
    Monochrome = 0,
    FullColor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmarenderpropertiesOracleOciomode {
    Disabled = 0,
    DisplayView = 1,
    Explicit = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmarenderpropertiesUsecacheratio {
    FixedSize = 0,
    ProportionOfPhysicalMemory = 1,
}

#[derive(Debug, Clone)]
pub struct LopKarmarenderproperties {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopKarmarenderproperties {
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
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: "Additional Render Vars"
    pub fn set_input_additional_render_vars<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Additional Render Vars" and specifies the output index of the target node.
    pub fn set_input_additional_render_vars_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_additional_render_vars_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_head_ao_distance(mut self, val: f32) -> Self {
        self.params.insert(
            "head_ao_distance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_ao_distance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_ao_distance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_depthcue_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "head_depthcue_alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_depthcue_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_depthcue_alpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffusequality(mut self, val: f32) -> Self {
        self.params.insert(
            "diffusequality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diffusequality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffusequality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflectquality(mut self, val: f32) -> Self {
        self.params.insert(
            "reflectquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflectquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflectquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractquality(mut self, val: f32) -> Self {
        self.params.insert(
            "refractquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refractquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumequality(mut self, val: f32) -> Self {
        self.params.insert(
            "volumequality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumequality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumequality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sssquality(mut self, val: f32) -> Self {
        self.params.insert(
            "sssquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sssquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sssquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_sampling_quality(mut self, val: f32) -> Self {
        self.params.insert(
            "light_sampling_quality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_light_sampling_quality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_sampling_quality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumesteprate(mut self, val: f32) -> Self {
        self.params.insert(
            "volumesteprate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumesteprate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumesteprate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuselimit(mut self, val: f32) -> Self {
        self.params.insert(
            "diffuselimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diffuselimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuselimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflectionlimit(mut self, val: f32) -> Self {
        self.params.insert(
            "reflectionlimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflectionlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflectionlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractionlimit(mut self, val: f32) -> Self {
        self.params.insert(
            "refractionlimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refractionlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractionlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumelimit(mut self, val: f32) -> Self {
        self.params.insert(
            "volumelimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumelimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumelimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssslimit(mut self, val: f32) -> Self {
        self.params.insert(
            "ssslimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ssslimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ssslimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorlimit(mut self, val: f32) -> Self {
        self.params.insert(
            "colorlimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colorlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorlimitindirect(mut self, val: f32) -> Self {
        self.params.insert(
            "colorlimitindirect".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colorlimitindirect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorlimitindirect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumevblurscale(mut self, val: f32) -> Self {
        self.params.insert(
            "volumevblurscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumevblurscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumevblurscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_causticsroughnessclamp(mut self, val: f32) -> Self {
        self.params.insert(
            "causticsroughnessclamp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_causticsroughnessclamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "causticsroughnessclamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raybias(mut self, val: f32) -> Self {
        self.params.insert(
            "raybias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_raybias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raybias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offscreenquality(mut self, val: f32) -> Self {
        self.params.insert(
            "offscreenquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_offscreenquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offscreenquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dicingqualityscale(mut self, val: f32) -> Self {
        self.params.insert(
            "dicingqualityscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dicingqualityscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dicingqualityscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pixelfiltersize(mut self, val: f32) -> Self {
        self.params.insert(
            "pixelfiltersize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pixelfiltersize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelfiltersize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_toe(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_toe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_toe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_toe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_shoulder(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_shoulder".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_shoulder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_shoulder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_utoe(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_utoe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_utoe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_utoe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_ushoulder(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_ushoulder".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_ushoulder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_ushoulder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_slope(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_slope".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_slope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_slope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_linear(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_linear".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_linear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_linear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_linearangle(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_linearangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_linearangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_linearangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_toelength(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_toelength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_toelength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_toelength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_shoulderlength(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_shoulderlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_shoulderlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_shoulderlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_shoulderangle(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_shoulderangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_shoulderangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_shoulderangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pixelaspectratio(mut self, val: f32) -> Self {
        self.params.insert(
            "pixelAspectRatio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pixelaspectratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelAspectRatio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_dwa_compression(mut self, val: f32) -> Self {
        self.params.insert(
            "image_dwa_compression".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_image_dwa_compression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_dwa_compression".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmzbias(mut self, val: f32) -> Self {
        self.params.insert(
            "dcmzbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dcmzbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmzbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_variance(mut self, val: f32) -> Self {
        self.params.insert(
            "oracle_variance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oracle_variance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_variance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varianceaa_thresh(mut self, val: f32) -> Self {
        self.params.insert(
            "varianceaa_thresh".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_varianceaa_thresh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "varianceaa_thresh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savefrequency(mut self, val: f32) -> Self {
        self.params.insert(
            "savefrequency".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_savefrequency_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savefrequency".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cacheratio(mut self, val: f32) -> Self {
        self.params.insert(
            "cacheratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cacheratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cacheratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_head_depthcue_z(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "head_depthcue_z".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_head_depthcue_z_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_depthcue_z".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_head_depthcue_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_depthcue_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_depthcue_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_depthcue_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_datawindowndc(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "dataWindowNDC".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_datawindowndc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataWindowNDC".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_samplesperpixel(mut self, val: i32) -> Self {
        self.params.insert(
            "samplesperpixel".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_samplesperpixel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplesperpixel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathtracedsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "pathtracedsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathtracedsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathtracedsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_ao_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "head_ao_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_head_ao_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_ao_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varianceaa_minsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "varianceaa_minsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_varianceaa_minsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "varianceaa_minsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varianceaa_maxsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "varianceaa_maxsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_varianceaa_maxsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "varianceaa_maxsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_screendoorlimit(mut self, val: i32) -> Self {
        self.params.insert(
            "screendoorlimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_screendoorlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "screendoorlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_russianroulette_cutoff(mut self, val: i32) -> Self {
        self.params.insert(
            "russianroulette_cutoff".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_russianroulette_cutoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "russianroulette_cutoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geosamples(mut self, val: i32) -> Self {
        self.params.insert(
            "geosamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_geosamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geosamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xformsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "xformsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_xformsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instance_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "instance_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_instance_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instance_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpeaovlimit(mut self, val: i32) -> Self {
        self.params.insert(
            "lpeaovlimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lpeaovlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpeaovlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cryptomatterank_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("cryptomatterank{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cryptomatterank_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("cryptomatterank{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmcompression(mut self, val: i32) -> Self {
        self.params.insert(
            "dcmcompression".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_dcmcompression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmcompression".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_minsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "oracle_minsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_oracle_minsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_minsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiding_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "guiding_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_guiding_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiding_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_progressivepasses(mut self, val: i32) -> Self {
        self.params.insert(
            "progressivepasses".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_progressivepasses_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "progressivepasses".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bucketsize(mut self, val: i32) -> Self {
        self.params.insert(
            "bucketsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bucketsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bucketsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecacheratio(mut self, val: LopKarmarenderpropertiesUsecacheratio) -> Self {
        self.params.insert(
            "usecacheratio".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_usecacheratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecacheratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachesize(mut self, val: i32) -> Self {
        self.params.insert(
            "cachesize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_resolution(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "resolution".to_string(),
            crate::core::types::ParamValue::Int2(val),
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

    // --- Menu parameters ---
    pub fn with_quicksetup(mut self, val: LopKarmarenderpropertiesQuicksetup) -> Self {
        self.params.insert(
            "quicksetup".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_quicksetup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quicksetup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolutionmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "resolutionMenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_resolutionmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolutionMenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_sampling_mode(mut self, val: i32) -> Self {
        self.params.insert(
            "light_sampling_mode".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_light_sampling_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_sampling_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mblur(mut self, val: LopKarmarenderpropertiesMblur) -> Self {
        self.params.insert(
            "mblur".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_curve_basis(mut self, val: i32) -> Self {
        self.params.insert(
            "curve_basis".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_curve_basis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curve_basis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmofsize(mut self, val: LopKarmarenderpropertiesDcmofsize) -> Self {
        self.params.insert(
            "dcmofsize".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dcmofsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmofsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_ociomode(mut self, val: LopKarmarenderpropertiesOracleOciomode) -> Self {
        self.params.insert(
            "oracle_ociomode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_oracle_ociomode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_ociomode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_tonemapcurve(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "tonemapcurve".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_tonemapcurve_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemapcurve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderproductsparentprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "renderproductsparentprimpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renderproductsparentprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderproductsparentprimpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rendervarsparentprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "rendervarsparentprimpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rendervarsparentprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rendervarsparentprimpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picture(mut self, val: &str) -> Self {
        self.params.insert(
            "picture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_picture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_res_mode(mut self, val: &str) -> Self {
        self.params.insert(
            "res_mode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_res_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_engine(mut self, val: &str) -> Self {
        self.params.insert(
            "engine".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_engine_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "engine".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vblur(mut self, val: &str) -> Self {
        self.params.insert(
            "vblur".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blurstyle(mut self, val: &str) -> Self {
        self.params.insert(
            "blurstyle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blurstyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blurstyle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instance_vblur(mut self, val: &str) -> Self {
        self.params.insert(
            "instance_vblur".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_instance_vblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instance_vblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_point_style(mut self, val: &str) -> Self {
        self.params.insert(
            "point_style".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_point_style_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "point_style".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curve_style(mut self, val: &str) -> Self {
        self.params.insert(
            "curve_style".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_curve_style_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curve_style".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cull_backface(mut self, val: &str) -> Self {
        self.params.insert(
            "cull_backface".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cull_backface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cull_backface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dicingcamera(mut self, val: &str) -> Self {
        self.params.insert(
            "dicingcamera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dicingcamera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dicingcamera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pixelfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "pixelfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pixelfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_omitlpes(mut self, val: &str) -> Self {
        self.params.insert(
            "omitlpes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_omitlpes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "omitlpes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geoprimpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "geoprimpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geoprimpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geoprimpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "outputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitPfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitpfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitPprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitpprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitp_camerafilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitP_camerafilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitp_camerafilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitP_camerafilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitp_cameraprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitP_cameraprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitp_cameraprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitP_cameraprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpzfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitPzfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitpzfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPzfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpzprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitPzprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitpzprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPzprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_elementfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "elementfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_elementfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_elementprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "elementprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_elementprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primidfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "primidfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primidfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primidfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primidprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "primidprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primidprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primidprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hituvfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hituvfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hituvfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hituvfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hituvprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hituvprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hituvprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hituvprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitnfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitnfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitnprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitnprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitngfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNgfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitngfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNgfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitngprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNgprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitngprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNgprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitn_facingratiofilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitN_facingratiofilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitn_facingratiofilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitN_facingratiofilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitn_facingratioprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitN_facingratioprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitn_facingratioprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitN_facingratioprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitng_facingratiofilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNg_facingratiofilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitng_facingratiofilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNg_facingratiofilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitng_facingratioprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNg_facingratioprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitng_facingratioprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNg_facingratioprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motionvectorsfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "motionvectorsfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_motionvectorsfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motionvectorsfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motionvectorsprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "motionvectorsprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_motionvectorsprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motionvectorsprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocityfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "velocityfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_velocityfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocityfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocityprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "velocityprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_velocityprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocityprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_format_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("format{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_format_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("format{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_datatype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("dataType{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_datatype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dataType{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcename_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("sourceName{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcename_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("sourceName{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcetype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("sourceType{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcetype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("sourceType{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("filter{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("filter{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cryptomattesidecar_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("cryptomattesidecar{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cryptomattesidecar_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("cryptomattesidecar{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcs_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("outputcs{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputcs_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("outputcs{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoiser(mut self, val: &str) -> Self {
        self.params.insert(
            "denoiser".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_denoiser_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoiser".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoise_aovs(mut self, val: &str) -> Self {
        self.params.insert(
            "denoise_aovs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_denoise_aovs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoise_aovs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap(mut self, val: &str) -> Self {
        self.params.insert(
            "tonemap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tonemap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_aovs(mut self, val: &str) -> Self {
        self.params.insert(
            "tonemap_aovs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tonemap_aovs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_aovs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_planes_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ocio_planes{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocio_planes_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_planes{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_outputspace_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ocio_outputspace{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocio_outputspace_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_outputspace{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_inputspace_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ocio_inputspace{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocio_inputspace_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_inputspace{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_looks_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ocio_looks{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocio_looks_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_looks{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aspectratioconformpolicy(mut self, val: &str) -> Self {
        self.params.insert(
            "aspectRatioConformPolicy".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aspectratioconformpolicy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aspectRatioConformPolicy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_artist(mut self, val: &str) -> Self {
        self.params.insert(
            "image_artist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_artist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_artist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_comment(mut self, val: &str) -> Self {
        self.params.insert(
            "image_comment".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_comment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_comment".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_hostname(mut self, val: &str) -> Self {
        self.params.insert(
            "image_hostname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_hostname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_hostname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_exr_compression(mut self, val: &str) -> Self {
        self.params.insert(
            "image_exr_compression".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_exr_compression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_exr_compression".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmfilename(mut self, val: &str) -> Self {
        self.params.insert(
            "dcmfilename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dcmfilename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmfilename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmvars(mut self, val: &str) -> Self {
        self.params.insert(
            "dcmvars".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dcmvars_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmvars".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_convergence_mode(mut self, val: &str) -> Self {
        self.params.insert(
            "convergence_mode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_convergence_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convergence_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_plane(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle_plane".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_plane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_plane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_ociodisplay(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle_ociodisplay".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_ociodisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_ociodisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_ocioview(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle_ocioview".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_ocioview_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_ocioview".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_ociocolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle_ociocolorspace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_ociocolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_ociocolorspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_productname(mut self, val: &str) -> Self {
        self.params.insert(
            "productName".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_productname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "productName".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imagemode(mut self, val: &str) -> Self {
        self.params.insert(
            "imagemode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_imagemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imagemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bucketorder(mut self, val: &str) -> Self {
        self.params.insert(
            "bucketorder".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bucketorder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bucketorder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportcomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "exportcomponents".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_exportcomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportcomponents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffusecomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "diffusecomponents".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_diffusecomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffusecomponents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractcomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "refractcomponents".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refractcomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractcomponents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumecomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "volumecomponents".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_volumecomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumecomponents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssscomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "ssscomponents".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ssscomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ssscomponents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_force_headlight(mut self, val: bool) -> Self {
        self.params.insert(
            "force_headlight".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_force_headlight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "force_headlight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_addfog(mut self, val: bool) -> Self {
        self.params.insert(
            "head_addfog".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_head_addfog_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_addfog".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorlimitindlinked(mut self, val: bool) -> Self {
        self.params.insert(
            "colorlimitindlinked".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_colorlimitindlinked_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorlimitindlinked".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledof(mut self, val: bool) -> Self {
        self.params.insert(
            "enabledof".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablemblur(mut self, val: bool) -> Self {
        self.params.insert(
            "enablemblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablemblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_samplesfromstage(mut self, val: bool) -> Self {
        self.params.insert(
            "samplesfromstage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_samplesfromstage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplesfromstage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disableimageblur(mut self, val: bool) -> Self {
        self.params.insert(
            "disableimageblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_disableimageblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "disableimageblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_causticsenable(mut self, val: bool) -> Self {
        self.params.insert(
            "causticsenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_causticsenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "causticsenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_autoraybias(mut self, val: bool) -> Self {
        self.params.insert(
            "autoraybias".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_autoraybias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "autoraybias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainmaxrough(mut self, val: bool) -> Self {
        self.params.insert(
            "constrainmaxrough".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constrainmaxrough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainmaxrough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedicingcamera(mut self, val: bool) -> Self {
        self.params.insert(
            "usedicingcamera".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedicingcamera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedicingcamera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importsecondaryinputvars(mut self, val: bool) -> Self {
        self.params.insert(
            "importsecondaryinputvars".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importsecondaryinputvars_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importsecondaryinputvars".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importsecondaryproducts(mut self, val: bool) -> Self {
        self.params.insert(
            "importsecondaryproducts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importsecondaryproducts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importsecondaryproducts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doomitlpes(mut self, val: bool) -> Self {
        self.params.insert(
            "doomitlpes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doomitlpes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doomitlpes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dogeolights(mut self, val: bool) -> Self {
        self.params.insert(
            "dogeolights".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dogeolights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dogeolights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dooutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "dooutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dooutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dooutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beauty(mut self, val: bool) -> Self {
        self.params.insert(
            "beauty".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beauty_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beauty".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "beautyperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beautyperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "beautyunshadowed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beautyunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyunshadowed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "beautyunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beautyunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadow(mut self, val: bool) -> Self {
        self.params.insert(
            "shadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "shadowperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadowperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseshadow(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseshadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseshadowperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseshadowperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseshadow(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseshadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseshadowperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseshadowperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseshadow(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseshadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseshadowperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseshadowperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedglossyreflection(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedglossyreflection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedglossyreflection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedglossyreflection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedglossyreflectionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedglossyreflectionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directglossyreflection(mut self, val: bool) -> Self {
        self.params.insert(
            "directglossyreflection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directglossyreflection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directglossyreflection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directglossyreflectionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directglossyreflectionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectglossyreflection(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectglossyreflection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectglossyreflection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectglossyreflection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectglossyreflectionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectglossyreflectionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossytransmission(mut self, val: bool) -> Self {
        self.params.insert(
            "glossytransmission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_glossytransmission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossytransmission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossytransmissionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "glossytransmissionperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_glossytransmissionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossytransmissionperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coat(mut self, val: bool) -> Self {
        self.params.insert(
            "coat".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "coatperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coatperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedemission(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directemission(mut self, val: bool) -> Self {
        self.params.insert(
            "directemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectemission(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visiblelights(mut self, val: bool) -> Self {
        self.params.insert(
            "visiblelights".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visiblelights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visiblelights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visiblelightsperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "visiblelightsperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visiblelightsperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visiblelightsperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedvolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedvolumeperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedvolumeperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "directvolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directvolumeperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directvolumeperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectvolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectvolumeperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectvolumeperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss(mut self, val: bool) -> Self {
        self.params.insert(
            "sss".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sss_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sssperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "sssperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sssperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sssperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_albedo(mut self, val: bool) -> Self {
        self.params.insert(
            "albedo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_albedo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "albedo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ambientocclusion(mut self, val: bool) -> Self {
        self.params.insert(
            "ambientocclusion".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ambientocclusion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ambientocclusion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitp(mut self, val: bool) -> Self {
        self.params.insert(
            "hitP".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitP".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitp_camera(mut self, val: bool) -> Self {
        self.params.insert(
            "hitP_camera".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitp_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitP_camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpz(mut self, val: bool) -> Self {
        self.params.insert(
            "hitPz".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitpz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_element(mut self, val: bool) -> Self {
        self.params.insert(
            "element".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_element_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "element".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primid(mut self, val: bool) -> Self {
        self.params.insert(
            "primid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_primid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hituv(mut self, val: bool) -> Self {
        self.params.insert(
            "hituv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hituv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hituv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitn(mut self, val: bool) -> Self {
        self.params.insert(
            "hitN".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitn_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitN".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitng(mut self, val: bool) -> Self {
        self.params.insert(
            "hitNg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitng_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitn_facingratio(mut self, val: bool) -> Self {
        self.params.insert(
            "hitN_facingratio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitn_facingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitN_facingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitng_facingratio(mut self, val: bool) -> Self {
        self.params.insert(
            "hitNg_facingratio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitng_facingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNg_facingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motionvectors(mut self, val: bool) -> Self {
        self.params.insert(
            "motionvectors".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_motionvectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motionvectors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_split_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("split{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_split_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("split{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cryptomatte_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("cryptomatte{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cryptomatte_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("cryptomatte{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dooutputcs_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("dooutputcs{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dooutputcs_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dooutputcs{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoise_usealbedo(mut self, val: bool) -> Self {
        self.params.insert(
            "denoise_usealbedo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_denoise_usealbedo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoise_usealbedo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoise_usen(mut self, val: bool) -> Self {
        self.params.insert(
            "denoise_useN".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_denoise_usen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoise_useN".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("ocio_enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ocio_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcm(mut self, val: bool) -> Self {
        self.params.insert(
            "dcm".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dcm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcm".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setlayerrendersettings(mut self, val: bool) -> Self {
        self.params.insert(
            "setlayerrendersettings".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setlayerrendersettings_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setlayerrendersettings".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiding_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "guiding_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guiding_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiding_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcheckpoint(mut self, val: bool) -> Self {
        self.params.insert(
            "outputcheckpoint".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputcheckpoint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputcheckpoint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resume(mut self, val: bool) -> Self {
        self.params.insert(
            "resume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_abortmissingtexture(mut self, val: bool) -> Self {
        self.params.insert(
            "abortmissingtexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_abortmissingtexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "abortmissingtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_abortnogpudevice(mut self, val: bool) -> Self {
        self.params.insert(
            "abortnogpudevice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_abortnogpudevice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "abortnogpudevice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopKarmarenderproperties {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "karmarenderproperties"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub enum LopKarmarendersettingsMblur {
    OnByDefault = 0,
    OffByDefault = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmarendersettingsOutlineMode {
    Determined = 0,
    Random = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmarendersettingsDcmofsize {
    Monochrome = 0,
    FullColor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmarendersettingsOracleOciomode {
    Disabled = 0,
    DisplayView = 1,
    Explicit = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmarendersettingsUsecacheratio {
    FixedSize = 0,
    ProportionOfPhysicalMemory = 1,
}

#[derive(Debug, Clone)]
pub struct LopKarmarendersettings {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopKarmarendersettings {
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
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: "Additional Render Vars"
    pub fn set_input_additional_render_vars<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Additional Render Vars" and specifies the output index of the target node.
    pub fn set_input_additional_render_vars_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_additional_render_vars_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(1),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_head_ao_distance(mut self, val: f32) -> Self {
        self.params.insert(
            "head_ao_distance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_ao_distance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_ao_distance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_depthcue_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "head_depthcue_alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_head_depthcue_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_depthcue_alpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffusequality(mut self, val: f32) -> Self {
        self.params.insert(
            "diffusequality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diffusequality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffusequality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflectquality(mut self, val: f32) -> Self {
        self.params.insert(
            "reflectquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflectquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflectquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractquality(mut self, val: f32) -> Self {
        self.params.insert(
            "refractquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refractquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumequality(mut self, val: f32) -> Self {
        self.params.insert(
            "volumequality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumequality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumequality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sssquality(mut self, val: f32) -> Self {
        self.params.insert(
            "sssquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sssquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sssquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_sampling_quality(mut self, val: f32) -> Self {
        self.params.insert(
            "light_sampling_quality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_light_sampling_quality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_sampling_quality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumesteprate(mut self, val: f32) -> Self {
        self.params.insert(
            "volumesteprate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumesteprate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumesteprate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuselimit(mut self, val: f32) -> Self {
        self.params.insert(
            "diffuselimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diffuselimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuselimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflectionlimit(mut self, val: f32) -> Self {
        self.params.insert(
            "reflectionlimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflectionlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflectionlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractionlimit(mut self, val: f32) -> Self {
        self.params.insert(
            "refractionlimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refractionlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractionlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumelimit(mut self, val: f32) -> Self {
        self.params.insert(
            "volumelimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumelimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumelimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssslimit(mut self, val: f32) -> Self {
        self.params.insert(
            "ssslimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ssslimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ssslimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorlimit(mut self, val: f32) -> Self {
        self.params.insert(
            "colorlimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colorlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorlimitindirect(mut self, val: f32) -> Self {
        self.params.insert(
            "colorlimitindirect".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colorlimitindirect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorlimitindirect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumevblurscale(mut self, val: f32) -> Self {
        self.params.insert(
            "volumevblurscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumevblurscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumevblurscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_causticsroughnessclamp(mut self, val: f32) -> Self {
        self.params.insert(
            "causticsroughnessclamp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_causticsroughnessclamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "causticsroughnessclamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raybias(mut self, val: f32) -> Self {
        self.params.insert(
            "raybias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_raybias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raybias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offscreenquality(mut self, val: f32) -> Self {
        self.params.insert(
            "offscreenquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_offscreenquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offscreenquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dicingqualityscale(mut self, val: f32) -> Self {
        self.params.insert(
            "dicingqualityscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dicingqualityscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dicingqualityscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "outline_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_outline_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_depththreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "outline_depththreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_outline_depththreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_depththreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "outline_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_outline_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_fitmin(mut self, val: f32) -> Self {
        self.params.insert(
            "outline_fitmin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_outline_fitmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_fitmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_fitmax(mut self, val: f32) -> Self {
        self.params.insert(
            "outline_fitmax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_outline_fitmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_fitmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_depthblend(mut self, val: f32) -> Self {
        self.params.insert(
            "outline_depthblend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_outline_depthblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_depthblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pixelfiltersize(mut self, val: f32) -> Self {
        self.params.insert(
            "pixelfiltersize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pixelfiltersize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelfiltersize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_holdout_shadow_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "holdout_shadow_alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_holdout_shadow_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "holdout_shadow_alpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_toe(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_toe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_toe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_toe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_shoulder(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_shoulder".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_shoulder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_shoulder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_utoe(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_utoe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_utoe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_utoe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_ushoulder(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_ushoulder".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_ushoulder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_ushoulder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_slope(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_slope".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_slope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_slope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_linear(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_linear".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_linear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_linear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_linearangle(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_linearangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_linearangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_linearangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_toelength(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_toelength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_toelength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_toelength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_shoulderlength(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_shoulderlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_shoulderlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_shoulderlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_shoulderangle(mut self, val: f32) -> Self {
        self.params.insert(
            "tonemap_shoulderangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tonemap_shoulderangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_shoulderangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pixelaspectratio(mut self, val: f32) -> Self {
        self.params.insert(
            "pixelAspectRatio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pixelaspectratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelAspectRatio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_dwa_compression(mut self, val: f32) -> Self {
        self.params.insert(
            "image_dwa_compression".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_image_dwa_compression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_dwa_compression".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_metadata_float_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("metadata_float_{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_metadata_float_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("metadata_float_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmzbias(mut self, val: f32) -> Self {
        self.params.insert(
            "dcmzbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dcmzbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmzbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_variance(mut self, val: f32) -> Self {
        self.params.insert(
            "oracle_variance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oracle_variance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_variance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varianceaa_thresh(mut self, val: f32) -> Self {
        self.params.insert(
            "varianceaa_thresh".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_varianceaa_thresh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "varianceaa_thresh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savefrequency(mut self, val: f32) -> Self {
        self.params.insert(
            "savefrequency".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_savefrequency_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savefrequency".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cacheratio(mut self, val: f32) -> Self {
        self.params.insert(
            "cacheratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cacheratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cacheratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_head_depthcue_z(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "head_depthcue_z".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_head_depthcue_z_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_depthcue_z".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_depthcue_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "head_depthcue_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_head_depthcue_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_depthcue_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "outline_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_outline_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_metadata_color_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("metadata_color_{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_metadata_color_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("metadata_color_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_datawindowndc(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "dataWindowNDC".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_datawindowndc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataWindowNDC".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_samplesperpixel(mut self, val: i32) -> Self {
        self.params.insert(
            "samplesperpixel".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_samplesperpixel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplesperpixel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathtracedsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "pathtracedsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathtracedsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathtracedsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_ao_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "head_ao_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_head_ao_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_ao_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varianceaa_minsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "varianceaa_minsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_varianceaa_minsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "varianceaa_minsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varianceaa_maxsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "varianceaa_maxsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_varianceaa_maxsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "varianceaa_maxsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_screendoorlimit(mut self, val: i32) -> Self {
        self.params.insert(
            "screendoorlimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_screendoorlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "screendoorlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_russianroulette_cutoff(mut self, val: i32) -> Self {
        self.params.insert(
            "russianroulette_cutoff".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_russianroulette_cutoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "russianroulette_cutoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geosamples(mut self, val: i32) -> Self {
        self.params.insert(
            "geosamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_geosamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geosamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xformsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "xformsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_xformsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instance_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "instance_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_instance_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instance_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_primary_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "outline_primary_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_outline_primary_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_primary_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_secondary_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "outline_secondary_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_outline_secondary_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_secondary_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_mode(mut self, val: LopKarmarendersettingsOutlineMode) -> Self {
        self.params.insert(
            "outline_mode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_outline_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_primary_stencils(mut self, val: i32) -> Self {
        self.params.insert(
            "outline_primary_stencils".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_outline_primary_stencils_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_primary_stencils".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_secondary_stencils(mut self, val: i32) -> Self {
        self.params.insert(
            "outline_secondary_stencils".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_outline_secondary_stencils_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_secondary_stencils".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpeaovlimit(mut self, val: i32) -> Self {
        self.params.insert(
            "lpeaovlimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lpeaovlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpeaovlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cryptomatterank_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("cryptomatterank{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cryptomatterank_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("cryptomatterank{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_metadata_int_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("metadata_int_{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_metadata_int_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("metadata_int_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmcompression(mut self, val: i32) -> Self {
        self.params.insert(
            "dcmcompression".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_dcmcompression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmcompression".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_minsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "oracle_minsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_oracle_minsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_minsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiding_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "guiding_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_guiding_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiding_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_progressivepasses(mut self, val: i32) -> Self {
        self.params.insert(
            "progressivepasses".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_progressivepasses_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "progressivepasses".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bucketsize(mut self, val: i32) -> Self {
        self.params.insert(
            "bucketsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bucketsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bucketsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecacheratio(mut self, val: LopKarmarendersettingsUsecacheratio) -> Self {
        self.params.insert(
            "usecacheratio".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_usecacheratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecacheratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachesize(mut self, val: i32) -> Self {
        self.params.insert(
            "cachesize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_resolution(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "resolution".to_string(),
            crate::core::types::ParamValue::Int2(val),
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

    // --- Menu parameters ---
    pub fn with_resolutionmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "resolutionMenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_resolutionmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolutionMenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_light_sampling_mode(mut self, val: i32) -> Self {
        self.params.insert(
            "light_sampling_mode".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_light_sampling_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_sampling_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mblur(mut self, val: LopKarmarendersettingsMblur) -> Self {
        self.params.insert(
            "mblur".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_curve_basis(mut self, val: i32) -> Self {
        self.params.insert(
            "curve_basis".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_curve_basis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curve_basis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmofsize(mut self, val: LopKarmarendersettingsDcmofsize) -> Self {
        self.params.insert(
            "dcmofsize".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dcmofsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmofsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_ociomode(mut self, val: LopKarmarendersettingsOracleOciomode) -> Self {
        self.params.insert(
            "oracle_ociomode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_oracle_ociomode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_ociomode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_tonemapcurve(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "tonemapcurve".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_tonemapcurve_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemapcurve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderproductsparentprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "renderproductsparentprimpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renderproductsparentprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderproductsparentprimpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rendervarsparentprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "rendervarsparentprimpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rendervarsparentprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rendervarsparentprimpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picture(mut self, val: &str) -> Self {
        self.params.insert(
            "picture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_picture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_res_mode(mut self, val: &str) -> Self {
        self.params.insert(
            "res_mode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_res_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_engine(mut self, val: &str) -> Self {
        self.params.insert(
            "engine".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_engine_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "engine".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vblur(mut self, val: &str) -> Self {
        self.params.insert(
            "vblur".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blurstyle(mut self, val: &str) -> Self {
        self.params.insert(
            "blurstyle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blurstyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blurstyle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instance_vblur(mut self, val: &str) -> Self {
        self.params.insert(
            "instance_vblur".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_instance_vblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instance_vblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_point_style(mut self, val: &str) -> Self {
        self.params.insert(
            "point_style".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_point_style_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "point_style".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curve_style(mut self, val: &str) -> Self {
        self.params.insert(
            "curve_style".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_curve_style_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curve_style".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cull_backface(mut self, val: &str) -> Self {
        self.params.insert(
            "cull_backface".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cull_backface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cull_backface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dicingcamera(mut self, val: &str) -> Self {
        self.params.insert(
            "dicingcamera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dicingcamera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dicingcamera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_existingvars(mut self, val: &str) -> Self {
        self.params.insert(
            "existingvars".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_existingvars_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "existingvars".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pixelfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "pixelfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pixelfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_omitlpes(mut self, val: &str) -> Self {
        self.params.insert(
            "omitlpes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_omitlpes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "omitlpes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "outputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitPfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitpfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitPprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitpprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitp_camerafilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitP_camerafilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitp_camerafilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitP_camerafilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitp_cameraprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitP_cameraprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitp_cameraprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitP_cameraprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpzfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitPzfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitpzfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPzfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpzprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitPzprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitpzprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPzprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_elementfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "elementfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_elementfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_elementprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "elementprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_elementprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primidfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "primidfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primidfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primidfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primidprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "primidprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primidprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primidprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hituvfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hituvfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hituvfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hituvfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hituvprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hituvprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hituvprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hituvprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitnfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitnfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitnprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitnprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitngfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNgfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitngfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNgfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitngprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNgprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitngprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNgprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitn_facingratiofilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitN_facingratiofilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitn_facingratiofilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitN_facingratiofilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitn_facingratioprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitN_facingratioprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitn_facingratioprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitN_facingratioprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitng_facingratiofilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNg_facingratiofilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitng_facingratiofilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNg_facingratiofilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitng_facingratioprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNg_facingratioprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitng_facingratioprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNg_facingratioprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motionvectorsfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "motionvectorsfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_motionvectorsfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motionvectorsfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motionvectorsprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "motionvectorsprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_motionvectorsprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motionvectorsprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocityfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "velocityfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_velocityfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocityfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocityprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "velocityprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_velocityprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocityprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_format_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("format{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_format_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("format{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_datatype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("dataType{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_datatype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dataType{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcename_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("sourceName{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcename_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("sourceName{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcetype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("sourceType{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcetype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("sourceType{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("filter{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("filter{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cryptomattesidecar_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("cryptomattesidecar{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cryptomattesidecar_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("cryptomattesidecar{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcs_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("outputcs{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputcs_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("outputcs{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_utilitypathexpression_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("utilitypathexpression{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_utilitypathexpression_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("utilitypathexpression{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoiser(mut self, val: &str) -> Self {
        self.params.insert(
            "denoiser".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_denoiser_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoiser".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoise_aovs(mut self, val: &str) -> Self {
        self.params.insert(
            "denoise_aovs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_denoise_aovs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoise_aovs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap(mut self, val: &str) -> Self {
        self.params.insert(
            "tonemap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tonemap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tonemap_aovs(mut self, val: &str) -> Self {
        self.params.insert(
            "tonemap_aovs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tonemap_aovs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tonemap_aovs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_planes_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ocio_planes{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocio_planes_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_planes{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_outputspace_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ocio_outputspace{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocio_outputspace_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_outputspace{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_inputspace_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ocio_inputspace{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocio_inputspace_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_inputspace{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_looks_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ocio_looks{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ocio_looks_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_looks{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aspectratioconformpolicy(mut self, val: &str) -> Self {
        self.params.insert(
            "aspectRatioConformPolicy".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aspectratioconformpolicy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aspectRatioConformPolicy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_artist(mut self, val: &str) -> Self {
        self.params.insert(
            "image_artist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_artist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_artist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_comment(mut self, val: &str) -> Self {
        self.params.insert(
            "image_comment".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_comment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_comment".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_hostname(mut self, val: &str) -> Self {
        self.params.insert(
            "image_hostname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_hostname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_hostname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_exr_compression(mut self, val: &str) -> Self {
        self.params.insert(
            "image_exr_compression".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_exr_compression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_exr_compression".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_metadata_type_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("metadata_type_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_metadata_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("metadata_type_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_metadata_key_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("metadata_key_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_metadata_key_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("metadata_key_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_metadata_string_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("metadata_string_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_metadata_string_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("metadata_string_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmfilename(mut self, val: &str) -> Self {
        self.params.insert(
            "dcmfilename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dcmfilename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmfilename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmvars(mut self, val: &str) -> Self {
        self.params.insert(
            "dcmvars".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dcmvars_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmvars".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_convergence_mode(mut self, val: &str) -> Self {
        self.params.insert(
            "convergence_mode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_convergence_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convergence_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_plane(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle_plane".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_plane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_plane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_ociodisplay(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle_ociodisplay".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_ociodisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_ociodisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_ocioview(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle_ocioview".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_ocioview_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_ocioview".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oracle_ociocolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "oracle_ociocolorspace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oracle_ociocolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oracle_ociocolorspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_productname(mut self, val: &str) -> Self {
        self.params.insert(
            "productName".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_productname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "productName".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imagemode(mut self, val: &str) -> Self {
        self.params.insert(
            "imagemode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_imagemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imagemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bucketorder(mut self, val: &str) -> Self {
        self.params.insert(
            "bucketorder".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bucketorder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bucketorder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportcomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "exportcomponents".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_exportcomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportcomponents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffusecomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "diffusecomponents".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_diffusecomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffusecomponents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractcomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "refractcomponents".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refractcomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractcomponents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumecomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "volumecomponents".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_volumecomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumecomponents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssscomponents(mut self, val: &str) -> Self {
        self.params.insert(
            "ssscomponents".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ssscomponents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ssscomponents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_force_headlight(mut self, val: bool) -> Self {
        self.params.insert(
            "force_headlight".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_force_headlight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "force_headlight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_head_addfog(mut self, val: bool) -> Self {
        self.params.insert(
            "head_addfog".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_head_addfog_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "head_addfog".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorlimitindlinked(mut self, val: bool) -> Self {
        self.params.insert(
            "colorlimitindlinked".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_colorlimitindlinked_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorlimitindlinked".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledof(mut self, val: bool) -> Self {
        self.params.insert(
            "enabledof".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablemblur(mut self, val: bool) -> Self {
        self.params.insert(
            "enablemblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablemblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_samplesfromstage(mut self, val: bool) -> Self {
        self.params.insert(
            "samplesfromstage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_samplesfromstage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplesfromstage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableimageblur(mut self, val: bool) -> Self {
        self.params.insert(
            "enableimageblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableimageblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableimageblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_causticsenable(mut self, val: bool) -> Self {
        self.params.insert(
            "causticsenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_causticsenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "causticsenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_autoraybias(mut self, val: bool) -> Self {
        self.params.insert(
            "autoraybias".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_autoraybias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "autoraybias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainmaxrough(mut self, val: bool) -> Self {
        self.params.insert(
            "constrainmaxrough".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constrainmaxrough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainmaxrough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedicingcamera(mut self, val: bool) -> Self {
        self.params.insert(
            "usedicingcamera".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedicingcamera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedicingcamera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "outline_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outline_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_apply(mut self, val: bool) -> Self {
        self.params.insert(
            "outline_apply".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outline_apply_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_apply".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_linesaovs(mut self, val: bool) -> Self {
        self.params.insert(
            "outline_linesaovs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outline_linesaovs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_linesaovs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_primary(mut self, val: bool) -> Self {
        self.params.insert(
            "outline_primary".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outline_primary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_primary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_secondary(mut self, val: bool) -> Self {
        self.params.insert(
            "outline_secondary".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outline_secondary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_secondary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_debug(mut self, val: bool) -> Self {
        self.params.insert(
            "outline_debug".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outline_debug_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_debug".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importsecondaryinputvars(mut self, val: bool) -> Self {
        self.params.insert(
            "importsecondaryinputvars".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importsecondaryinputvars_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importsecondaryinputvars".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importsecondaryproducts(mut self, val: bool) -> Self {
        self.params.insert(
            "importsecondaryproducts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importsecondaryproducts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importsecondaryproducts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doomitlpes(mut self, val: bool) -> Self {
        self.params.insert(
            "doomitlpes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doomitlpes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doomitlpes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dooutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "dooutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dooutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dooutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_channel_lower_rgb(mut self, val: bool) -> Self {
        self.params.insert(
            "channel_lower_rgb".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_channel_lower_rgb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "channel_lower_rgb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beauty(mut self, val: bool) -> Self {
        self.params.insert(
            "beauty".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beauty_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beauty".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "beautyperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beautyperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "beautyunshadowed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beautyunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyunshadowed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "beautyunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beautyunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadow(mut self, val: bool) -> Self {
        self.params.insert(
            "shadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "shadowperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadowperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseshadow(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseshadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseshadowperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseshadowperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseshadow(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseshadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseshadowperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseshadowperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseshadow(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseshadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseshadowperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseshadowperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedglossyreflection(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedglossyreflection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedglossyreflection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedglossyreflection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedglossyreflectionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedglossyreflectionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directglossyreflection(mut self, val: bool) -> Self {
        self.params.insert(
            "directglossyreflection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directglossyreflection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directglossyreflection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directglossyreflectionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directglossyreflectionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectglossyreflection(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectglossyreflection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectglossyreflection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectglossyreflection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectglossyreflectionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectglossyreflectionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossytransmission(mut self, val: bool) -> Self {
        self.params.insert(
            "glossytransmission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_glossytransmission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossytransmission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossytransmissionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "glossytransmissionperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_glossytransmissionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossytransmissionperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coat(mut self, val: bool) -> Self {
        self.params.insert(
            "coat".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "coatperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coatperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedemission(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directemission(mut self, val: bool) -> Self {
        self.params.insert(
            "directemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectemission(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visiblelights(mut self, val: bool) -> Self {
        self.params.insert(
            "visiblelights".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visiblelights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visiblelights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visiblelightsperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "visiblelightsperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visiblelightsperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visiblelightsperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedvolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedvolumeperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedvolumeperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "directvolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directvolumeperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directvolumeperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectvolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectvolumeperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectvolumeperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss(mut self, val: bool) -> Self {
        self.params.insert(
            "sss".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sss_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sssperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "sssperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sssperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sssperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_albedo(mut self, val: bool) -> Self {
        self.params.insert(
            "albedo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_albedo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "albedo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ambientocclusion(mut self, val: bool) -> Self {
        self.params.insert(
            "ambientocclusion".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ambientocclusion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ambientocclusion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitp(mut self, val: bool) -> Self {
        self.params.insert(
            "hitP".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitP".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitp_camera(mut self, val: bool) -> Self {
        self.params.insert(
            "hitP_camera".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitp_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitP_camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpz(mut self, val: bool) -> Self {
        self.params.insert(
            "hitPz".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitpz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_element(mut self, val: bool) -> Self {
        self.params.insert(
            "element".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_element_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "element".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primid(mut self, val: bool) -> Self {
        self.params.insert(
            "primid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_primid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hituv(mut self, val: bool) -> Self {
        self.params.insert(
            "hituv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hituv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hituv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitn(mut self, val: bool) -> Self {
        self.params.insert(
            "hitN".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitn_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitN".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitng(mut self, val: bool) -> Self {
        self.params.insert(
            "hitNg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitng_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitn_facingratio(mut self, val: bool) -> Self {
        self.params.insert(
            "hitN_facingratio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitn_facingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitN_facingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitng_facingratio(mut self, val: bool) -> Self {
        self.params.insert(
            "hitNg_facingratio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitng_facingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNg_facingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motionvectors(mut self, val: bool) -> Self {
        self.params.insert(
            "motionvectors".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_motionvectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motionvectors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_split_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("split{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_split_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("split{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cryptomatte_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("cryptomatte{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cryptomatte_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("cryptomatte{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dooutputcs_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("dooutputcs{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dooutputcs_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dooutputcs{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_holdout_shadows(mut self, val: bool) -> Self {
        self.params.insert(
            "holdout_shadows".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_holdout_shadows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "holdout_shadows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoise_usealbedo(mut self, val: bool) -> Self {
        self.params.insert(
            "denoise_usealbedo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_denoise_usealbedo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoise_usealbedo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoise_usen(mut self, val: bool) -> Self {
        self.params.insert(
            "denoise_useN".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_denoise_usen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoise_useN".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoise_cpu_only(mut self, val: bool) -> Self {
        self.params.insert(
            "denoise_cpu_only".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_denoise_cpu_only_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoise_cpu_only".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ocio_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("ocio_enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ocio_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ocio_enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcm(mut self, val: bool) -> Self {
        self.params.insert(
            "dcm".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dcm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcm".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmusehitdist(mut self, val: bool) -> Self {
        self.params.insert(
            "dcmusehitdist".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dcmusehitdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmusehitdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dcmexcludeholdouts(mut self, val: bool) -> Self {
        self.params.insert(
            "dcmexcludeholdouts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dcmexcludeholdouts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcmexcludeholdouts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setlayerrendersettings(mut self, val: bool) -> Self {
        self.params.insert(
            "setlayerrendersettings".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setlayerrendersettings_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setlayerrendersettings".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiding_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "guiding_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guiding_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiding_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcheckpoint(mut self, val: bool) -> Self {
        self.params.insert(
            "outputcheckpoint".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputcheckpoint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputcheckpoint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resume(mut self, val: bool) -> Self {
        self.params.insert(
            "resume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_abortmissingtexture(mut self, val: bool) -> Self {
        self.params.insert(
            "abortmissingtexture".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_abortmissingtexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "abortmissingtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_abortnogpudevice(mut self, val: bool) -> Self {
        self.params.insert(
            "abortnogpudevice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_abortnogpudevice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "abortnogpudevice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopKarmarendersettings {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "karmarendersettings"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub enum LopKarmaskyatmosphereCreateprims {
    Edit = 0,
    Create = 1,
    /// Force Edit (Ignore Editable Flag)
    ForceEditIgnoreEditableFlag = 2,
}

#[derive(Debug, Clone)]
pub struct LopKarmaskyatmosphere {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopKarmaskyatmosphere {
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
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_rayleighscale(mut self, val: f32) -> Self {
        self.params.insert(
            "rayleighscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rayleighscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rayleighscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rayleighdist(mut self, val: f32) -> Self {
        self.params.insert(
            "rayleighdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rayleighdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rayleighdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsrayleighdist_p8a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarsrayleighdist_p8a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarsrayleighdist_p8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsrayleighdist_p8a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_miescatterscale(mut self, val: f32) -> Self {
        self.params.insert(
            "miescatterscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_miescatterscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "miescatterscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mieabsorpscale(mut self, val: f32) -> Self {
        self.params.insert(
            "mieabsorpscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mieabsorpscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mieabsorpscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_miedist(mut self, val: f32) -> Self {
        self.params.insert(
            "miedist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_miedist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "miedist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmiedist_k0a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarsmiedist_k0a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarsmiedist_k0a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsmiedist_k0a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmiephase_61a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarsmiephase_61a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarsmiephase_61a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsmiephase_61a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extrascale(mut self, val: f32) -> Self {
        self.params.insert(
            "extrascale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_extrascale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extrascale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extraaltitude(mut self, val: f32) -> Self {
        self.params.insert(
            "extraaltitude".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_extraaltitude_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extraaltitude".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsextraaltitude_cbb(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarsextraaltitude_cbb".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarsextraaltitude_cbb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsextraaltitude_cbb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extrawidth(mut self, val: f32) -> Self {
        self.params.insert(
            "extrawidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_extrawidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extrawidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsextrawidth_g5a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarsextrawidth_g5a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarsextrawidth_g5a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsextrawidth_g5a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sealevel(mut self, val: f32) -> Self {
        self.params.insert(
            "sealevel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sealevel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sealevel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarssealevel_61a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarssealevel_61a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarssealevel_61a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarssealevel_61a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unitscale(mut self, val: f32) -> Self {
        self.params.insert(
            "unitscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unitscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unitscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stepsizemin(mut self, val: f32) -> Self {
        self.params.insert(
            "stepsizemin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stepsizemin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stepsizemin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsstepsizemin_26a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarsstepsizemin_26a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarsstepsizemin_26a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsstepsizemin_26a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stepsizemax(mut self, val: f32) -> Self {
        self.params.insert(
            "stepsizemax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stepsizemax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stepsizemax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsstepsizemax_26a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarsstepsizemax_26a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarsstepsizemax_26a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsstepsizemax_26a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundoffset_p8a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarsgroundoffset_p8a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarsgroundoffset_p8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundoffset_p8a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarslutdim_xya(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "xn__primvarslutdim_xya".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_xn_primvarslutdim_xya_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarslutdim_xya".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rayleighcol(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rayleighcol".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rayleighcol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rayleighcol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsrayleighcoeff_cbb(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__primvarsrayleighcoeff_cbb".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_primvarsrayleighcoeff_cbb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsrayleighcoeff_cbb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_miescattercol(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "miescattercol".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_miescattercol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "miescattercol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmiescattercoeff_leb(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__primvarsmiescattercoeff_leb".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_primvarsmiescattercoeff_leb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsmiescattercoeff_leb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mieabsorpcol(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "mieabsorpcol".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_mieabsorpcol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mieabsorpcol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmieabsorpcoeff_ycb(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__primvarsmieabsorpcoeff_ycb".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_primvarsmieabsorpcoeff_ycb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsmieabsorpcoeff_ycb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extraabsorpcol(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "extraabsorpcol".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_extraabsorpcol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extraabsorpcol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsextraabsorpcoeff_7fb(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__primvarsextraabsorpcoeff_7fb".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_primvarsextraabsorpcoeff_7fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsextraabsorpcoeff_7fb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundalbedo_p8a(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__primvarsgroundalbedo_p8a".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_primvarsgroundalbedo_p8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundalbedo_p8a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_planetcenter(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "planetcenter".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_planetcenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planetcenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsplanetcenter_p8a(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__primvarsplanetcenter_p8a".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_primvarsplanetcenter_p8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsplanetcenter_p8a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primcount(mut self, val: i32) -> Self {
        self.params.insert(
            "primcount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primcount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarskarmaobjectvolumelimit_zpbfg(mut self, val: i32) -> Self {
        self.params.insert(
            "xn__primvarskarmaobjectvolumelimit_zpbfg".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_xn_primvarskarmaobjectvolumelimit_zpbfg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarskarmaobjectvolumelimit_zpbfg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_createprims(mut self, val: LopKarmaskyatmosphereCreateprims) -> Self {
        self.params.insert(
            "createprims".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initforedit(mut self, val: i32) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_initforedit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primtype(mut self, val: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primkind(mut self, val: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specifier(mut self, val: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_specifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_classancestor(mut self, val: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_classancestor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsenableatmosphere_control_8sb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsenableatmosphere_control_8sb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsenableatmosphere_control_8sb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsenableatmosphere_control_8sb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rayleighcoeff_control(mut self, val: &str) -> Self {
        self.params.insert(
            "rayleighcoeff_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rayleighcoeff_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rayleighcoeff_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsrayleighdist_control_qmb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsrayleighdist_control_qmb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsrayleighdist_control_qmb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsrayleighdist_control_qmb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmiecoeff_control_7fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsmiecoeff_control_7fb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmiecoeff_control_7fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsmiecoeff_control_7fb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmieabsorpcoeff_control_zpb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsmieabsorpcoeff_control_zpb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmieabsorpcoeff_control_zpb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsmieabsorpcoeff_control_zpb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmiedist_control_leb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsmiedist_control_leb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmiedist_control_leb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsmiedist_control_leb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmiephase_control_7fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsmiephase_control_7fb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsmiephase_control_7fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsmiephase_control_7fb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsextraabsorption_control_mrb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsextraabsorption_control_mrb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsextraabsorption_control_mrb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsextraabsorption_control_mrb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsextraaltitude_control_dob(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsextraaltitude_control_dob".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsextraaltitude_control_dob_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsextraaltitude_control_dob".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsextrawidth_control_hjb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsextrawidth_control_hjb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsextrawidth_control_hjb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsextrawidth_control_hjb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundalbedo_control_qmb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundalbedo_control_qmb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundalbedo_control_qmb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundalbedo_control_qmb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundvisibility_control_8sb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundvisibility_control_8sb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundvisibility_control_8sb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundvisibility_control_8sb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundvisibility_7fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundvisibility_7fb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundvisibility_7fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundvisibility_7fb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsplanetcenter_control_qmb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsplanetcenter_control_qmb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsplanetcenter_control_qmb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsplanetcenter_control_qmb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarssealevel_control_7fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarssealevel_control_7fb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarssealevel_control_7fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarssealevel_control_7fb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarskarmaobjectlpetag_control_vubfg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarskarmaobjectlpetag_control_vubfg".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarskarmaobjectlpetag_control_vubfg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarskarmaobjectlpetag_control_vubfg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarskarmaobjectlpetag_uhbfg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarskarmaobjectlpetag_uhbfg".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarskarmaobjectlpetag_uhbfg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarskarmaobjectlpetag_uhbfg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsstepsizemin_control_3kb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsstepsizemin_control_3kb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsstepsizemin_control_3kb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsstepsizemin_control_3kb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsstepsizemax_control_3kb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsstepsizemax_control_3kb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsstepsizemax_control_3kb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsstepsizemax_control_3kb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarskarmaobjectvolumelimit_control_02bfg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarskarmaobjectvolumelimit_control_02bfg".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarskarmaobjectvolumelimit_control_02bfg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarskarmaobjectvolumelimit_control_02bfg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsociofrom_control_7fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsociofrom_control_7fb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsociofrom_control_7fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsociofrom_control_7fb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsociofrom_61a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsociofrom_61a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsociofrom_61a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsociofrom_61a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarslutdim_control_ycb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarslutdim_control_ycb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarslutdim_control_ycb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarslutdim_control_ycb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundoffset_control_qmb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundoffset_control_qmb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundoffset_control_qmb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundoffset_control_qmb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundreceiveshadow_control_4xb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundreceiveshadow_control_4xb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundreceiveshadow_control_4xb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundreceiveshadow_control_4xb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsrenderbackside_control_zpb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsrenderbackside_control_zpb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsrenderbackside_control_zpb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsrenderbackside_control_zpb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarssolidalphainipr_control_mrb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarssolidalphainipr_control_mrb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarssolidalphainipr_control_mrb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarssolidalphainipr_control_mrb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computeextents(mut self, val: bool) -> Self {
        self.params.insert(
            "computeextents".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computeextents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computeextents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsenableatmosphere_7fb(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__primvarsenableatmosphere_7fb".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_primvarsenableatmosphere_7fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsenableatmosphere_7fb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsgroundreceiveshadow_3kb(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__primvarsgroundreceiveshadow_3kb".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_primvarsgroundreceiveshadow_3kb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsgroundreceiveshadow_3kb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsrenderbackside_ycb(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__primvarsrenderbackside_ycb".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_primvarsrenderbackside_ycb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsrenderbackside_ycb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarssolidalphainipr_leb(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__primvarssolidalphainipr_leb".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_primvarssolidalphainipr_leb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarssolidalphainipr_leb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopKarmaskyatmosphere {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "karmaskyatmosphere"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub enum LopKarmaskydomelightCreateprims {
    Edit = 0,
    Create = 1,
    /// Force Edit (Ignore Editable Flag)
    ForceEditIgnoreEditableFlag = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmaskydomelightXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmaskydomelightRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopKarmaskydomelight {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopKarmaskydomelight {
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
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
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
    pub fn with_xn_inputssolar_altitude_n8a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputssolar_altitude_n8a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputssolar_altitude_n8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputssolar_altitude_n8a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputssolar_azimuth_06a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputssolar_azimuth_06a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputssolar_azimuth_06a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputssolar_azimuth_06a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsturbidity_i0a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsturbidity_i0a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsturbidity_i0a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsturbidity_i0a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputshorizon_blur_e5a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputshorizon_blur_e5a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputshorizon_blur_e5a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputshorizon_blur_e5a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsintensity_i0a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsintensity_i0a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsintensity_i0a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsintensity_i0a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsexposure_vya(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsexposure_vya".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsexposure_vya_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsexposure_vya".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputscolortemperature_wcb(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputscolorTemperature_wcb".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputscolortemperature_wcb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputscolorTemperature_wcb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsdiffuse_8wa(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsdiffuse_8wa".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsdiffuse_8wa_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsdiffuse_8wa".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsspecular_vya(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsspecular_vya".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsspecular_vya_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsspecular_vya".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_houdiniguidescale_s3a(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__houdiniguidescale_s3a".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_houdiniguidescale_s3a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniguidescale_s3a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowdistance_n8ag(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__inputsshadowdistance_n8ag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_inputsshadowdistance_n8ag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowdistance_n8ag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
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
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shear".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shear".to_string(),
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
    pub fn with_xn_inputsground_albedo_06a(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__inputsground_albedo_06a".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_inputsground_albedo_06a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsground_albedo_06a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsground_color_e5a(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__inputsground_color_e5a".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_inputsground_color_e5a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsground_color_e5a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputscolor_zta(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__inputscolor_zta".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_inputscolor_zta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputscolor_zta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowcolor_r3ag(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__inputsshadowcolor_r3ag".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_inputsshadowcolor_r3ag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowcolor_r3ag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_createprims(mut self, val: LopKarmaskydomelightCreateprims) -> Self {
        self.params.insert(
            "createprims".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initforedit(mut self, val: i32) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_initforedit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: LopKarmaskydomelightXord) -> Self {
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
    pub fn with_rord(mut self, val: LopKarmaskydomelightRord) -> Self {
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

    // --- String parameters ---
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primtype(mut self, val: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specifier(mut self, val: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_specifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_classancestor(mut self, val: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_classancestor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputssolar_altitude_control_omb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputssolar_altitude_control_omb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputssolar_altitude_control_omb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputssolar_altitude_control_omb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputssolar_azimuth_control_1kb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputssolar_azimuth_control_1kb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputssolar_azimuth_control_1kb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputssolar_azimuth_control_1kb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsturbidity_control_jeb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsturbidity_control_jeb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsturbidity_control_jeb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsturbidity_control_jeb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsground_albedo_control_1kb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsground_albedo_control_1kb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsground_albedo_control_1kb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsground_albedo_control_1kb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsground_color_control_fjb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsground_color_control_fjb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsground_color_control_fjb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsground_color_control_fjb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputshorizon_blur_control_fjb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputshorizon_blur_control_fjb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputshorizon_blur_control_fjb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputshorizon_blur_control_fjb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsintensity_control_jeb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsintensity_control_jeb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsintensity_control_jeb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsintensity_control_jeb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsexposure_control_wcb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsexposure_control_wcb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsexposure_control_wcb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsexposure_control_wcb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputscolor_control_06a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputscolor_control_06a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputscolor_control_06a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputscolor_control_06a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsenablecolortemperature_control_pzb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsenableColorTemperature_control_pzb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsenablecolortemperature_control_pzb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsenableColorTemperature_control_pzb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputscolortemperature_control_xpb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputscolorTemperature_control_xpb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputscolortemperature_control_xpb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputscolorTemperature_control_xpb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filters_control(mut self, val: &str) -> Self {
        self.params.insert(
            "filters_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filters_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filters_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filters(mut self, val: &str) -> Self {
        self.params.insert(
            "filters".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filters_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filters".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_portals_control(mut self, val: &str) -> Self {
        self.params.insert(
            "portals_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_portals_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "portals_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_portals(mut self, val: &str) -> Self {
        self.params.insert(
            "portals".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_portals_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "portals".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsnormalize_control_jeb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsnormalize_control_jeb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsnormalize_control_jeb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsnormalize_control_jeb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsdiffuse_control_99a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsdiffuse_control_99a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsdiffuse_control_99a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsdiffuse_control_99a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsspecular_control_wcb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsspecular_control_wcb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsspecular_control_wcb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsspecular_control_wcb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_houdiniguidescale_control_thb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__houdiniguidescale_control_thb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_houdiniguidescale_control_thb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniguidescale_control_thb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_houdiniinviewermenu_control_2kb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__houdiniinviewermenu_control_2kb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_houdiniinviewermenu_control_2kb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniinviewermenu_control_2kb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_lightfilters_control_m8a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__lightfilters_control_m8a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_lightfilters_control_m8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__lightfilters_control_m8a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_lightfilters_lva(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__lightfilters_lva".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_lightfilters_lva_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__lightfilters_lva".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowenable_control_fjbg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowenable_control_fjbg".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowenable_control_fjbg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowenable_control_fjbg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowcolor_control_shbg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowcolor_control_shbg".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowcolor_control_shbg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowcolor_control_shbg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowdistance_control_ombg(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowdistance_control_ombg".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowdistance_control_ombg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowdistance_control_ombg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsenablecolortemperature_omb(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__inputsenableColorTemperature_omb".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_inputsenablecolortemperature_omb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsenableColorTemperature_omb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsnormalize_i0a(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__inputsnormalize_i0a".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_inputsnormalize_i0a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsnormalize_i0a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_houdiniinviewermenu_16a(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__houdiniinviewermenu_16a".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_houdiniinviewermenu_16a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__houdiniinviewermenu_16a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_inputsshadowenable_e5ag(mut self, val: bool) -> Self {
        self.params.insert(
            "xn__inputsshadowenable_e5ag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xn_inputsshadowenable_e5ag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__inputsshadowenable_e5ag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopKarmaskydomelight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "karmaskydomelight"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub struct LopKarmastandardrendervars {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopKarmastandardrendervars {
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
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_lpeaovlimit(mut self, val: i32) -> Self {
        self.params.insert(
            "lpeaovlimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lpeaovlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpeaovlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_parentprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_omitlpes(mut self, val: &str) -> Self {
        self.params.insert(
            "omitlpes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_omitlpes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "omitlpes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "beautyfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_beautyfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "beautyprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_beautyprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "beautyoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_beautyoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyunshadowedfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "beautyunshadowedfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_beautyunshadowedfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyunshadowedfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyunshadowedprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "beautyunshadowedprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_beautyunshadowedprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyunshadowedprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyunshadowedoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "beautyunshadowedoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_beautyunshadowedoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyunshadowedoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "shadowfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shadowfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "shadowprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shadowprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "shadowoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shadowoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffusefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "combineddiffusefilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combineddiffusefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffusefilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "combineddiffuseprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "combineddiffuseoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffusefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "directdiffusefilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directdiffusefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffusefilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "directdiffuseprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directdiffuseprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "directdiffuseoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directdiffuseoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffusefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectdiffusefilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectdiffusefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffusefilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectdiffuseprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectdiffuseoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseunshadowedfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "combineddiffuseunshadowedfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseunshadowedfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseunshadowedfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseunshadowedprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "combineddiffuseunshadowedprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseunshadowedprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseunshadowedprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseunshadowedoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "combineddiffuseunshadowedoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseunshadowedoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseunshadowedoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseunshadowedfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "directdiffuseunshadowedfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directdiffuseunshadowedfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseunshadowedfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseunshadowedprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "directdiffuseunshadowedprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directdiffuseunshadowedprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseunshadowedprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseunshadowedoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "directdiffuseunshadowedoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directdiffuseunshadowedoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseunshadowedoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowedfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowedfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowedfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowedfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowedprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowedprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowedprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowedprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowedoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowedoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowedoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowedoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseshadowfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "combineddiffuseshadowfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseshadowfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseshadowfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseshadowprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "combineddiffuseshadowprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseshadowprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseshadowprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseshadowoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "combineddiffuseshadowoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseshadowoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseshadowoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseshadowfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "directdiffuseshadowfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directdiffuseshadowfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseshadowfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseshadowprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "directdiffuseshadowprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directdiffuseshadowprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseshadowprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseshadowoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "directdiffuseshadowoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directdiffuseshadowoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseshadowoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseshadowfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectdiffuseshadowfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseshadowfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseshadowfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseshadowprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectdiffuseshadowprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseshadowprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseshadowprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseshadowoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectdiffuseshadowoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseshadowoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseshadowoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedglossyreflectionfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "combinedglossyreflectionfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combinedglossyreflectionfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedglossyreflectionfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedglossyreflectionprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "combinedglossyreflectionprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combinedglossyreflectionprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedglossyreflectionprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedglossyreflectionoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "combinedglossyreflectionoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combinedglossyreflectionoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedglossyreflectionoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directglossyreflectionfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "directglossyreflectionfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directglossyreflectionfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directglossyreflectionfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directglossyreflectionprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "directglossyreflectionprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directglossyreflectionprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directglossyreflectionprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directglossyreflectionoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "directglossyreflectionoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directglossyreflectionoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directglossyreflectionoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectglossyreflectionfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectglossyreflectionfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectglossyreflectionfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectglossyreflectionfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectglossyreflectionprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectglossyreflectionprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectglossyreflectionprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectglossyreflectionprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectglossyreflectionoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectglossyreflectionoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectglossyreflectionoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectglossyreflectionoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossytransmissionfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "glossytransmissionfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_glossytransmissionfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossytransmissionfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossytransmissionprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "glossytransmissionprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_glossytransmissionprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossytransmissionprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossytransmissionoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "glossytransmissionoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_glossytransmissionoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossytransmissionoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "coatfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "coatprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "coatoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visiblelightsfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "visiblelightsfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_visiblelightsfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visiblelightsfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visiblelightsprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "visiblelightsprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_visiblelightsprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visiblelightsprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visiblelightsoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "visiblelightsoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_visiblelightsoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visiblelightsoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedemissionfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "combinedemissionfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combinedemissionfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedemissionfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedemissionprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "combinedemissionprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combinedemissionprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedemissionprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedemissionoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "combinedemissionoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combinedemissionoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedemissionoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directemissionfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "directemissionfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directemissionfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directemissionfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directemissionprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "directemissionprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directemissionprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directemissionprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directemissionoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "directemissionoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directemissionoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directemissionoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectemissionfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectemissionfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectemissionfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectemissionfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectemissionprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectemissionprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectemissionprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectemissionprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectemissionoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectemissionoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectemissionoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectemissionoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedvolumefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "combinedvolumefilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combinedvolumefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedvolumefilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedvolumeprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "combinedvolumeprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combinedvolumeprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedvolumeprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedvolumeoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "combinedvolumeoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_combinedvolumeoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedvolumeoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directvolumefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "directvolumefilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directvolumefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directvolumefilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directvolumeprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "directvolumeprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directvolumeprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directvolumeprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directvolumeoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "directvolumeoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directvolumeoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directvolumeoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectvolumefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectvolumefilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectvolumefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectvolumefilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectvolumeprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectvolumeprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectvolumeprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectvolumeprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectvolumeoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "indirectvolumeoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indirectvolumeoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectvolumeoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sssfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "sssfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sssfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sssfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sssprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "sssprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sssprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sssprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sssoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "sssoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sssoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sssoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_albedofilter(mut self, val: &str) -> Self {
        self.params.insert(
            "albedofilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_albedofilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "albedofilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_albedoprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "albedoprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_albedoprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "albedoprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_albedooutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "albedooutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_albedooutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "albedooutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ambientocclusionfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "ambientocclusionfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ambientocclusionfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ambientocclusionfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ambientocclusionprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "ambientocclusionprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ambientocclusionprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ambientocclusionprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ambientocclusionoutputcs(mut self, val: &str) -> Self {
        self.params.insert(
            "ambientocclusionoutputcs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ambientocclusionoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ambientocclusionoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitPfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitpfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitPprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitpprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitp_camerafilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitP_camerafilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitp_camerafilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitP_camerafilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitp_cameraprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitP_cameraprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitp_cameraprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitP_cameraprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpzfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitPzfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitpzfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPzfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpzprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitPzprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitpzprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPzprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_elementfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "elementfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_elementfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_elementprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "elementprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_elementprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elementprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primidfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "primidfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primidfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primidfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primidprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "primidprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primidprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primidprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hituvfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hituvfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hituvfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hituvfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hituvprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hituvprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hituvprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hituvprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitnfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitnfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitnprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitnprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitngfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNgfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitngfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNgfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitngprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNgprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitngprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNgprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitn_facingratiofilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitN_facingratiofilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitn_facingratiofilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitN_facingratiofilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitn_facingratioprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitN_facingratioprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitn_facingratioprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitN_facingratioprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitng_facingratiofilter(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNg_facingratiofilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitng_facingratiofilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNg_facingratiofilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitng_facingratioprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "hitNg_facingratioprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hitng_facingratioprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNg_facingratioprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motionvectorsfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "motionvectorsfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_motionvectorsfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motionvectorsfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motionvectorsprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "motionvectorsprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_motionvectorsprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motionvectorsprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocityfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "velocityfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_velocityfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocityfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocityprecision(mut self, val: &str) -> Self {
        self.params.insert(
            "velocityprecision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_velocityprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocityprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_doomitlpes(mut self, val: bool) -> Self {
        self.params.insert(
            "doomitlpes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doomitlpes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doomitlpes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beauty(mut self, val: bool) -> Self {
        self.params.insert(
            "beauty".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beauty_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beauty".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dobeautyoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "dobeautyoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dobeautyoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dobeautyoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "beautyperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beautyperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "beautyunshadowed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beautyunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyunshadowed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dobeautyunshadowedoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "dobeautyunshadowedoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dobeautyunshadowedoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dobeautyunshadowedoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beautyunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "beautyunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_beautyunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beautyunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadow(mut self, val: bool) -> Self {
        self.params.insert(
            "shadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doshadowoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "doshadowoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doshadowoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doshadowoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "shadowperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shadowperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docombineddiffuseoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "docombineddiffuseoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docombineddiffuseoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docombineddiffuseoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodirectdiffuseoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "dodirectdiffuseoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodirectdiffuseoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dodirectdiffuseoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doindirectdiffuseoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "doindirectdiffuseoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doindirectdiffuseoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doindirectdiffuseoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docombineddiffuseunshadowedoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "docombineddiffuseunshadowedoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docombineddiffuseunshadowedoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docombineddiffuseunshadowedoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodirectdiffuseunshadowedoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "dodirectdiffuseunshadowedoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodirectdiffuseunshadowedoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dodirectdiffuseunshadowedoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowed(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doindirectdiffuseunshadowedoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "doindirectdiffuseunshadowedoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doindirectdiffuseunshadowedoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doindirectdiffuseunshadowedoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowedperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseunshadowedperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseunshadowedperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseshadow(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseshadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docombineddiffuseshadowoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "docombineddiffuseshadowoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docombineddiffuseshadowoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docombineddiffuseshadowoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combineddiffuseshadowperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combineddiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combineddiffuseshadowperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combineddiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseshadow(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseshadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodirectdiffuseshadowoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "dodirectdiffuseshadowoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodirectdiffuseshadowoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dodirectdiffuseshadowoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuseshadowperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directdiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directdiffuseshadowperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseshadow(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseshadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doindirectdiffuseshadowoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "doindirectdiffuseshadowoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doindirectdiffuseshadowoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doindirectdiffuseshadowoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuseshadowperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectdiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectdiffuseshadowperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuseshadowperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedglossyreflection(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedglossyreflection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedglossyreflection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedglossyreflection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docombinedglossyreflectionoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "docombinedglossyreflectionoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docombinedglossyreflectionoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docombinedglossyreflectionoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedglossyreflectionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedglossyreflectionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directglossyreflection(mut self, val: bool) -> Self {
        self.params.insert(
            "directglossyreflection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directglossyreflection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directglossyreflection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodirectglossyreflectionoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "dodirectglossyreflectionoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodirectglossyreflectionoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dodirectglossyreflectionoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directglossyreflectionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directglossyreflectionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectglossyreflection(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectglossyreflection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectglossyreflection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectglossyreflection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doindirectglossyreflectionoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "doindirectglossyreflectionoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doindirectglossyreflectionoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doindirectglossyreflectionoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectglossyreflectionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectglossyreflectionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectglossyreflectionperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossytransmission(mut self, val: bool) -> Self {
        self.params.insert(
            "glossytransmission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_glossytransmission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossytransmission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doglossytransmissionoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "doglossytransmissionoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doglossytransmissionoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doglossytransmissionoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossytransmissionperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "glossytransmissionperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_glossytransmissionperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossytransmissionperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coat(mut self, val: bool) -> Self {
        self.params.insert(
            "coat".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docoatoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "docoatoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docoatoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docoatoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "coatperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coatperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visiblelights(mut self, val: bool) -> Self {
        self.params.insert(
            "visiblelights".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visiblelights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visiblelights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dovisiblelightsoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "dovisiblelightsoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dovisiblelightsoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dovisiblelightsoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visiblelightsperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "visiblelightsperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visiblelightsperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visiblelightsperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedemission(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docombinedemissionoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "docombinedemissionoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docombinedemissionoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docombinedemissionoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directemission(mut self, val: bool) -> Self {
        self.params.insert(
            "directemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodirectemissionoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "dodirectemissionoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodirectemissionoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dodirectemissionoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectemission(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doindirectemissionoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "doindirectemissionoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doindirectemissionoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doindirectemissionoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedvolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docombinedvolumeoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "docombinedvolumeoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docombinedvolumeoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docombinedvolumeoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinedvolumeperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "combinedvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_combinedvolumeperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinedvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "directvolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodirectvolumeoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "dodirectvolumeoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodirectvolumeoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dodirectvolumeoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directvolumeperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "directvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directvolumeperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectvolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doindirectvolumeoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "doindirectvolumeoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doindirectvolumeoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doindirectvolumeoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectvolumeperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "indirectvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indirectvolumeperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectvolumeperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss(mut self, val: bool) -> Self {
        self.params.insert(
            "sss".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sss_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dosssoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "dosssoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosssoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dosssoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sssperlpe(mut self, val: bool) -> Self {
        self.params.insert(
            "sssperlpe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sssperlpe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sssperlpe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_albedo(mut self, val: bool) -> Self {
        self.params.insert(
            "albedo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_albedo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "albedo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doalbedooutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "doalbedooutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doalbedooutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doalbedooutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ambientocclusion(mut self, val: bool) -> Self {
        self.params.insert(
            "ambientocclusion".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ambientocclusion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ambientocclusion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doambientocclusionoutputcs(mut self, val: bool) -> Self {
        self.params.insert(
            "doambientocclusionoutputcs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doambientocclusionoutputcs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doambientocclusionoutputcs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitp(mut self, val: bool) -> Self {
        self.params.insert(
            "hitP".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitP".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitp_camera(mut self, val: bool) -> Self {
        self.params.insert(
            "hitP_camera".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitp_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitP_camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitpz(mut self, val: bool) -> Self {
        self.params.insert(
            "hitPz".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitpz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitPz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_element(mut self, val: bool) -> Self {
        self.params.insert(
            "element".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_element_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "element".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primid(mut self, val: bool) -> Self {
        self.params.insert(
            "primid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_primid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hituv(mut self, val: bool) -> Self {
        self.params.insert(
            "hituv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hituv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hituv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitn(mut self, val: bool) -> Self {
        self.params.insert(
            "hitN".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitn_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitN".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitng(mut self, val: bool) -> Self {
        self.params.insert(
            "hitNg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitng_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitn_facingratio(mut self, val: bool) -> Self {
        self.params.insert(
            "hitN_facingratio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitn_facingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitN_facingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hitng_facingratio(mut self, val: bool) -> Self {
        self.params.insert(
            "hitNg_facingratio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hitng_facingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hitNg_facingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motionvectors(mut self, val: bool) -> Self {
        self.params.insert(
            "motionvectors".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_motionvectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motionvectors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopKarmastandardrendervars {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "karmastandardrendervars"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub enum LopKarmatexturebakerResolutionMenu {
    /// 256 x 256
    N256X256 = 0,
    /// 512 x 512
    N512X512 = 1,
    /// 1024 x 1024
    N1024X1024 = 2,
    /// 2048 x 2048
    N2048X2048 = 3,
    /// 4096 x 4096
    N4096X4096 = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmatexturebakerRnMode {
    ConcaveAndConvex = 0,
    Concave = 1,
    Convex = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmatexturebakerEgMode {
    ConcaveAndConvex = 0,
    Concave = 1,
    Convex = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopKarmatexturebakerTruedisplace {
    BumpMappedDisplacementShaders = 0,
    TrueDisplacement = 1,
    DisableDisplacementShaders = 2,
}

#[derive(Debug, Clone)]
pub struct LopKarmatexturebaker {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopKarmatexturebaker {
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
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_rn_raybias(mut self, val: f32) -> Self {
        self.params.insert(
            "rn_raybias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rn_raybias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rn_raybias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rn_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "rn_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rn_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rn_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rn_cuspangle(mut self, val: f32) -> Self {
        self.params.insert(
            "rn_cuspangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rn_cuspangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rn_cuspangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oc_maxdist(mut self, val: f32) -> Self {
        self.params.insert(
            "oc_maxdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oc_maxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oc_maxdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oc_raybias(mut self, val: f32) -> Self {
        self.params.insert(
            "oc_raybias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oc_raybias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oc_raybias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eg_raybias(mut self, val: f32) -> Self {
        self.params.insert(
            "eg_raybias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eg_raybias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eg_raybias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eg_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "eg_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eg_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eg_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eg_cuspangle(mut self, val: f32) -> Self {
        self.params.insert(
            "eg_cuspangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eg_cuspangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eg_cuspangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tracebias(mut self, val: f32) -> Self {
        self.params.insert(
            "tracebias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tracebias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tracebias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dicingquality(mut self, val: f32) -> Self {
        self.params.insert(
            "dicingquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dicingquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dicingquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_pboxmin(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Pboxmin".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pboxmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Pboxmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pboxmax(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Pboxmax".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pboxmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Pboxmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_default_udim(mut self, val: i32) -> Self {
        self.params.insert(
            "default_udim".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_default_udim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "default_udim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rn_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "rn_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_rn_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rn_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rn_mode(mut self, val: LopKarmatexturebakerRnMode) -> Self {
        self.params.insert(
            "rn_mode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_rn_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rn_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oc_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "oc_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_oc_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oc_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cu_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "cu_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cu_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cu_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cv_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "cv_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cv_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cv_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_th_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "th_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_th_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "th_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eg_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "eg_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_eg_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eg_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eg_mode(mut self, val: LopKarmatexturebakerEgMode) -> Self {
        self.params.insert(
            "eg_mode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_eg_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eg_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_samplesperpixel(mut self, val: i32) -> Self {
        self.params.insert(
            "samplesperpixel".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_samplesperpixel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplesperpixel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_resolution(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "resolution".to_string(),
            crate::core::types::ParamValue::Int2(val),
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

    // --- Menu parameters ---
    pub fn with_resolution_menu(mut self, val: LopKarmatexturebakerResolutionMenu) -> Self {
        self.params.insert(
            "resolution_menu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_resolution_menu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolution_menu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_truedisplace(mut self, val: LopKarmatexturebakerTruedisplace) -> Self {
        self.params.insert(
            "truedisplace".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_truedisplace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "truedisplace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bakemode(mut self, val: &str) -> Self {
        self.params.insert(
            "bakemode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bakemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bakemesh(mut self, val: &str) -> Self {
        self.params.insert(
            "bakemesh".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bakemesh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakemesh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cagemesh(mut self, val: &str) -> Self {
        self.params.insert(
            "cagemesh".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cagemesh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cagemesh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_highmesh(mut self, val: &str) -> Self {
        self.params.insert(
            "highmesh".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_highmesh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "highmesh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picture(mut self, val: &str) -> Self {
        self.params.insert(
            "picture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_picture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_uv(mut self, val: &str) -> Self {
        self.params.insert(
            "bake_uv".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bake_uv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_uv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_format(mut self, val: &str) -> Self {
        self.params.insert(
            "image_format".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_format_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "image_format".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name_separator(mut self, val: &str) -> Self {
        self.params.insert(
            "name_separator".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name_separator_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name_separator".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoiser(mut self, val: &str) -> Self {
        self.params.insert(
            "denoiser".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_denoiser_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoiser".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_denoiser_aovs(mut self, val: &str) -> Self {
        self.params.insert(
            "denoiser_aovs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_denoiser_aovs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "denoiser_aovs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flipn(mut self, val: &str) -> Self {
        self.params.insert(
            "flipN".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_flipn_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flipN".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_range_nt(mut self, val: &str) -> Self {
        self.params.insert(
            "range_Nt".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_range_nt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "range_Nt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_nt_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_Nt_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_nt_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Nt_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_rn_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_Rn_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_rn_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Rn_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_worldspace(mut self, val: &str) -> Self {
        self.params.insert(
            "worldspace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_worldspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worldspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_p_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_P_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_p_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_P_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pboxmode(mut self, val: &str) -> Self {
        self.params.insert(
            "Pboxmode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pboxmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Pboxmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_n_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_N_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_n_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_N_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_range_n(mut self, val: &str) -> Self {
        self.params.insert(
            "range_N".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_range_n_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "range_N".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_ng_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_Ng_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_ng_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Ng_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_range_ng(mut self, val: &str) -> Self {
        self.params.insert(
            "range_Ng".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_range_ng_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "range_Ng".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_oc_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_Oc_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_oc_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Oc_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_cu_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_Cu_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_cu_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Cu_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_cv_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_Cv_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_cv_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Cv_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_th_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_Th_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_th_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Th_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_eg_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_Eg_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_eg_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Eg_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_ds_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_Ds_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_ds_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Ds_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_vd_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_Vd_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_vd_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Vd_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_vdt_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_Vdt_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_vdt_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Vdt_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_af_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_Af_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_af_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Af_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_base_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_base_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_base_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_base_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_base_color_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_base_color_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_base_color_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_base_color_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_base_combined_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_base_combined_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_base_combined_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_base_combined_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_diffuse_roughness_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_diffuse_roughness_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_diffuse_roughness_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_diffuse_roughness_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_metalness_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_metalness_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_metalness_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_metalness_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_specular_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_specular_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_specular_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_specular_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_specular_color_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_specular_color_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_specular_color_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_specular_color_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_specular_roughness_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_specular_roughness_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_specular_roughness_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_specular_roughness_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_specular_ior_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_specular_IOR_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_specular_ior_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_specular_IOR_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_specular_anisotropy_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_specular_anisotropy_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_specular_anisotropy_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_specular_anisotropy_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_specular_rotation_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_specular_rotation_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_specular_rotation_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_specular_rotation_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_coat_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_coat_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_color_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_coat_color_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_coat_color_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_color_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_roughness_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_coat_roughness_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_coat_roughness_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_roughness_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_anisotropy_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_coat_anisotropy_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_coat_anisotropy_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_anisotropy_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_rotation_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_coat_rotation_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_coat_rotation_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_rotation_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_ior_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_coat_IOR_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_coat_ior_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_IOR_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_normal_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_coat_normal_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_coat_normal_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_normal_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_affect_color_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_coat_affect_color_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_coat_affect_color_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_affect_color_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_affect_roughness_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_coat_affect_roughness_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_coat_affect_roughness_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_affect_roughness_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_transmission_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_transmission_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_color_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_transmission_color_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_color_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_transmission_color_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_depth_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_transmission_depth_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_depth_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_transmission_depth_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_scatter_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_transmission_scatter_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_scatter_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_transmission_scatter_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_scatter_anisotropy_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_transmission_scatter_anisotropy_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_scatter_anisotropy_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_transmission_scatter_anisotropy_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_dispersion_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_transmission_dispersion_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_dispersion_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_transmission_dispersion_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_extra_roughness_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_transmission_extra_roughness_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_extra_roughness_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_transmission_extra_roughness_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_subsurface_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_subsurface_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_subsurface_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_subsurface_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_subsurface_color_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_subsurface_color_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_subsurface_color_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_subsurface_color_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_subsurface_radius_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_subsurface_radius_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_subsurface_radius_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_subsurface_radius_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_subsurface_scale_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_subsurface_scale_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_subsurface_scale_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_subsurface_scale_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_subsurface_anisotropy_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_subsurface_anisotropy_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_subsurface_anisotropy_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_subsurface_anisotropy_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_sheen_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_sheen_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_sheen_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_sheen_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_sheen_color_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_sheen_color_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_sheen_color_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_sheen_color_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_sheen_roughness_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_sheen_roughness_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_sheen_roughness_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_sheen_roughness_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_thin_film_thickness_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_thin_film_thickness_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_thin_film_thickness_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_thin_film_thickness_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_thin_film_ior_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_thin_film_IOR_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_thin_film_ior_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_thin_film_IOR_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_emission_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_emission_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_emission_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_emission_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_emission_color_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_emission_color_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_emission_color_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_emission_color_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_opacity_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_opacity_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_opacity_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_opacity_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_thin_walled_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_thin_walled_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_thin_walled_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_thin_walled_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_normal_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_normal_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_normal_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_normal_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_tangent_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_tangent_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_tangent_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_tangent_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_c_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_C_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_c_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_C_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_combineddiffuse_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_combineddiffuse_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_combineddiffuse_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_combineddiffuse_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_directdiffuse_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_directdiffuse_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_directdiffuse_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_directdiffuse_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_indirectdiffuse_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_indirectdiffuse_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_indirectdiffuse_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_indirectdiffuse_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_facingratio_n_name(mut self, val: &str) -> Self {
        self.params.insert(
            "aov_facingratio_N_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_facingratio_n_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_facingratio_N_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_extra_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("aov_extra_name{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_extra_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("aov_extra_name{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_extra_type_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("aov_extra_type{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_extra_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("aov_extra_type{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_extra_sourcename_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("aov_extra_sourcename{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_extra_sourcename_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("aov_extra_sourcename{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_extra_sourcetype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("aov_extra_sourcetype{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aov_extra_sourcetype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("aov_extra_sourcetype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_traceset(mut self, val: &str) -> Self {
        self.params.insert(
            "bake_traceset".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bake_traceset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_traceset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_missing_normals(mut self, val: &str) -> Self {
        self.params.insert(
            "missing_normals".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_missing_normals_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "missing_normals".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_traceset_remain(mut self, val: &str) -> Self {
        self.params.insert(
            "traceset_remain".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_traceset_remain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "traceset_remain".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_traceset_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("traceset_name{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_traceset_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("traceset_name{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_traceset_visibility_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("traceset_visibility{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_traceset_visibility_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("traceset_visibility{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_traceset_prims_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("traceset_prims{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_traceset_prims_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("traceset_prims{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_separate(mut self, val: bool) -> Self {
        self.params.insert(
            "separate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_separate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "separate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expand_islands(mut self, val: bool) -> Self {
        self.params.insert(
            "expand_islands".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_expand_islands_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expand_islands".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_nt(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_Nt".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_nt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Nt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_rn(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_Rn".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_rn_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Rn".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_p(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_P".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_P".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pboxuniform(mut self, val: bool) -> Self {
        self.params.insert(
            "Pboxuniform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pboxuniform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Pboxuniform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_n(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_N".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_n_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_N".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_ng(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_Ng".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_ng_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Ng".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_oc(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_Oc".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_oc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Oc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_cu(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_Cu".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_cu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Cu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_cv(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_Cv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_cv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Cv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_th(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_Th".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_th_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Th".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_eg(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_Eg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_eg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Eg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_ds(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_Ds".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_ds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Ds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_vd(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_Vd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_vd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Vd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_vdt(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_Vdt".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_vdt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Vdt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_af(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_Af".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_af_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_Af".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_base(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_base".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_base_color(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_base_color".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_base_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_base_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_base_combined(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_base_combined".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_base_combined_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_base_combined".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_diffuse_roughness(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_diffuse_roughness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_diffuse_roughness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_diffuse_roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_metalness(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_metalness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_metalness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_metalness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_specular(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_specular".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_specular_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_specular".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_specular_color(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_specular_color".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_specular_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_specular_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_specular_roughness(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_specular_roughness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_specular_roughness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_specular_roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_specular_ior(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_specular_IOR".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_specular_ior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_specular_IOR".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_specular_anisotropy(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_specular_anisotropy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_specular_anisotropy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_specular_anisotropy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_specular_rotation(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_specular_rotation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_specular_rotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_specular_rotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_coat".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_coat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_color(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_coat_color".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_coat_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_roughness(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_coat_roughness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_coat_roughness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_anisotropy(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_coat_anisotropy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_coat_anisotropy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_anisotropy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_rotation(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_coat_rotation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_coat_rotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_rotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_ior(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_coat_IOR".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_coat_ior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_IOR".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_normal(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_coat_normal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_coat_normal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_normal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_affect_color(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_coat_affect_color".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_coat_affect_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_affect_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_coat_affect_roughness(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_coat_affect_roughness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_coat_affect_roughness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_coat_affect_roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_transmission(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_transmission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_transmission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_transmission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_color(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_transmission_color".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_transmission_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_transmission_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_depth(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_transmission_depth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_transmission_depth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_transmission_depth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_scatter(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_transmission_scatter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_transmission_scatter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_transmission_scatter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_scatter_anisotropy(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_transmission_scatter_anisotropy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_transmission_scatter_anisotropy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_transmission_scatter_anisotropy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_dispersion(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_transmission_dispersion".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_transmission_dispersion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_transmission_dispersion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_transmission_extra_roughness(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_transmission_extra_roughness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_transmission_extra_roughness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_transmission_extra_roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_subsurface(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_subsurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_subsurface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_subsurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_subsurface_color(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_subsurface_color".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_subsurface_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_subsurface_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_subsurface_radius(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_subsurface_radius".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_subsurface_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_subsurface_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_subsurface_scale(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_subsurface_scale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_subsurface_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_subsurface_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_subsurface_anisotropy(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_subsurface_anisotropy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_subsurface_anisotropy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_subsurface_anisotropy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_sheen(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_sheen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_sheen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_sheen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_sheen_color(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_sheen_color".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_sheen_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_sheen_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_sheen_roughness(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_sheen_roughness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_sheen_roughness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_sheen_roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_thin_film_thickness(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_thin_film_thickness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_thin_film_thickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_thin_film_thickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_thin_film_ior(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_thin_film_IOR".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_thin_film_ior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_thin_film_IOR".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_emission(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_emission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_emission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_emission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_emission_color(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_emission_color".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_emission_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_emission_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_opacity(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_opacity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_opacity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_opacity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_thin_walled(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_thin_walled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_thin_walled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_thin_walled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_normal(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_normal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_normal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_normal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_tangent(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_tangent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_tangent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_tangent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_lighting(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_lighting".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_lighting_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_lighting".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_c(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_C".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_c_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_C".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_combineddiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_combineddiffuse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_combineddiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_combineddiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_directdiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_directdiffuse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_directdiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_directdiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_indirectdiffuse(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_indirectdiffuse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_indirectdiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_indirectdiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_facingratio_n(mut self, val: bool) -> Self {
        self.params.insert(
            "aov_facingratio_N".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_facingratio_n_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aov_facingratio_N".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aov_extra_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("aov_extra_enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aov_extra_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("aov_extra_enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hide_bake_mesh(mut self, val: bool) -> Self {
        self.params.insert(
            "hide_bake_mesh".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hide_bake_mesh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hide_bake_mesh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use_mikkt(mut self, val: bool) -> Self {
        self.params.insert(
            "use_mikkt".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_mikkt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_mikkt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_traceset_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("traceset_enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_traceset_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("traceset_enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopKarmatexturebaker {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "karmatexturebaker"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub struct LopKarmatoonshading {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopKarmatoonshading {
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
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_difftex_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "difftex_blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_difftex_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "difftex_blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_difframp_min(mut self, val: f32) -> Self {
        self.params.insert(
            "difframp_min".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_difframp_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "difframp_min".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_difframp_max(mut self, val: f32) -> Self {
        self.params.insert(
            "difframp_max".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_difframp_max_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "difframp_max".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossramp_min(mut self, val: f32) -> Self {
        self.params.insert(
            "glossramp_min".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glossramp_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossramp_min".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossramp_max(mut self, val: f32) -> Self {
        self.params.insert(
            "glossramp_max".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glossramp_max_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossramp_max".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatramp_min(mut self, val: f32) -> Self {
        self.params.insert(
            "coatramp_min".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coatramp_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatramp_min".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatramp_max(mut self, val: f32) -> Self {
        self.params.insert(
            "coatramp_max".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coatramp_max_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatramp_max".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_difframp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "difframp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_difframp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "difframp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "glossramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_glossramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "coatramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_coatramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderproductsparentprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "renderproductsparentprimpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renderproductsparentprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderproductsparentprimpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rendervarsparentprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "rendervarsparentprimpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rendervarsparentprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rendervarsparentprimpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cvex_shader(mut self, val: &str) -> Self {
        self.params.insert(
            "cvex_shader".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cvex_shader_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cvex_shader".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_outline_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "outline_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outline_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply_albedo(mut self, val: bool) -> Self {
        self.params.insert(
            "apply_albedo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_apply_albedo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply_albedo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outline_glossy(mut self, val: bool) -> Self {
        self.params.insert(
            "outline_glossy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outline_glossy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_glossy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopKarmatoonshading {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "karmatoonshading"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub struct LopKinefxSopcharacterimport {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopKinefxSopcharacterimport {
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
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(index),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("f".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_animposepath(mut self, val: &str) -> Self {
        self.params.insert(
            "animposepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animposepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animposepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeopath(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeopath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeopath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeopath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_captureposepath(mut self, val: &str) -> Self {
        self.params.insert(
            "captureposepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_captureposepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "captureposepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "behavior".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "behavior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cliprangemode(mut self, val: &str) -> Self {
        self.params.insert(
            "cliprangemode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cliprangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cliprangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skelprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "skelprimpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skelprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skelprimpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "animprimpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animprimpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animsavepath(mut self, val: &str) -> Self {
        self.params.insert(
            "animsavepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animsavepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animsavepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geosavepath(mut self, val: &str) -> Self {
        self.params.insert(
            "geosavepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geosavepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geosavepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindmaterials(mut self, val: &str) -> Self {
        self.params.insert(
            "bindmaterials".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindmaterials_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindmaterials".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_attribs(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeo_attribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeo_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_indexattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeo_indexattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeo_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_indexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_constantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeo_constantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeo_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_constantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_scalarconstantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeo_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeo_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_partitionattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeo_partitionattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeo_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_partitionattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_subsetgroups(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeo_subsetgroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeo_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_subsetgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_customattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeo_customattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeo_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_customattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_attribs(mut self, val: &str) -> Self {
        self.params.insert(
            "animgeo_attribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animgeo_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_constantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "animgeo_constantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animgeo_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_constantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_scalarconstantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "animgeo_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animgeo_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_staticattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "animgeo_staticattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animgeo_staticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_staticattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_customattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "animgeo_customattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animgeo_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_customattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_shiftstart(mut self, val: bool) -> Self {
        self.params.insert(
            "shiftstart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shiftstart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shiftstart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setprimpath(mut self, val: bool) -> Self {
        self.params.insert(
            "setprimpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setprimpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setskelprimpath(mut self, val: bool) -> Self {
        self.params.insert(
            "setskelprimpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setskelprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setskelprimpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setanimprimpath(mut self, val: bool) -> Self {
        self.params.insert(
            "setanimprimpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setanimprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setanimprimpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_animsavepath(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_animsavepath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_animsavepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_animsavepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_geosavepath(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_geosavepath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_geosavepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_geosavepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importrestgeo(mut self, val: bool) -> Self {
        self.params.insert(
            "importrestgeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importrestgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importrestgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enablepolygonsassubd(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enablepolygonsassubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enablepolygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enablepolygonsassubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_polygonsassubd(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_polygonsassubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_polygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_polygonsassubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enableattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enableattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enableattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enableattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enableindexattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enableindexattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enableindexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enableindexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enableconstantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enableconstantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enableconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enableconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enablescalarconstantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enablescalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enablescalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enablescalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enablepartitionattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enablepartitionattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enablepartitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enablepartitionattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enableprefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enableprefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enableprefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enableprefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enablesubsetgroups(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enablesubsetgroups".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enablesubsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enablesubsetgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enablecustomattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enablecustomattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enablecustomattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enablecustomattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enabletranslateuvtost(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enabletranslateuvtost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enabletranslateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enabletranslateuvtost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_translateuvtost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_translateuvtost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_enableattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "animgeo_enableattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animgeo_enableattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_enableattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_enableconstantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "animgeo_enableconstantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animgeo_enableconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_enableconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_enablescalarconstantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "animgeo_enablescalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animgeo_enablescalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_enablescalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_enablestaticattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "animgeo_enablestaticattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animgeo_enablestaticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_enablestaticattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_enablecustomattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "animgeo_enablecustomattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animgeo_enablecustomattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_enablecustomattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopKinefxSopcharacterimport {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "kinefx::sopcharacterimport"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
