#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopUndulatusDstOffsettype {
    /// Per-Component Control
    PerMinusComponentControl = 0,
}

#[derive(Debug, Clone)]
pub struct VopUndulatus {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUndulatus {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_p_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_p<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("P".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_element_size_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_elementsize<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("elementsize".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_phase_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_phase<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("phase".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_relative_thickness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_thickness<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("thickness".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_blur_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_blur<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("blur".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_mirror_pattern_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_mirror<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("mirror".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_distort_wave_pattern_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_distortwaves<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("distortwaves".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_distortion_noise_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_custom_distortnoise<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("custom_distortnoise".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_amplitude_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dst_amplitude<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dst_amplitude".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_element_size_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dst_elementsizescale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dst_elementsizescale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_max_octaves_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dst_oct<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dst_oct".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(11),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dst_rough<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dst_rough".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_distortion_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(12),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dst_distort<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dst_distort".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_coverage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(13),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_coverage<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("coverage".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_lacunarity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(14),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dst_lac<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dst_lac".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_stretch_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(15),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dst_stretch<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dst_stretch".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_do_droop_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(16),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dst_dodroop<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dst_dodroop".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_droop_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(17),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dst_droop<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dst_droop".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_droop_direction_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(18),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dst_droopdir<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dst_droopdir".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_offset_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(19),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dst_offsetv<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dst_offsetv".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_offset_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(20),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dst_offset<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dst_offset".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_rotation_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(21),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rotation<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rotation".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_up_vector_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(22),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_upvector<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("upvector".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_coverage(mut self, val: f32) -> Self {
        self.params.insert(
            "coverage".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coverage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coverage".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_elementsize(mut self, val: f32) -> Self {
        self.params.insert(
            "elementsize".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
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
    pub fn with_phase(mut self, val: f32) -> Self {
        self.params.insert(
            "phase".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_phase_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "phase".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_thickness(mut self, val: f32) -> Self {
        self.params.insert(
            "thickness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_thickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thickness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blur(mut self, val: f32) -> Self {
        self.params.insert(
            "blur".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blur".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_custom_distortnoise(mut self, val: f32) -> Self {
        self.params.insert(
            "custom_distortnoise".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_custom_distortnoise_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "custom_distortnoise".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dst_amplitude(mut self, val: f32) -> Self {
        self.params.insert(
            "dst_amplitude".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dst_amplitude_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dst_amplitude".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dst_elementsizescale(mut self, val: f32) -> Self {
        self.params.insert(
            "dst_elementsizescale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dst_elementsizescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dst_elementsizescale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dst_offset(mut self, val: f32) -> Self {
        self.params.insert(
            "dst_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dst_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dst_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dst_lac(mut self, val: f32) -> Self {
        self.params.insert(
            "dst_lac".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dst_lac_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dst_lac".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dst_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "dst_rough".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dst_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dst_rough".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dst_distort(mut self, val: f32) -> Self {
        self.params.insert(
            "dst_distort".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dst_distort_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dst_distort".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dst_droop(mut self, val: f32) -> Self {
        self.params.insert(
            "dst_droop".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dst_droop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dst_droop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rotation(mut self, val: f32) -> Self {
        self.params.insert(
            "rotation".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "P".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "P".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dst_offsetv(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dst_offsetv".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dst_offsetv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dst_offsetv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dst_stretch(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dst_stretch".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dst_stretch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dst_stretch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dst_droopdir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dst_droopdir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dst_droopdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dst_droopdir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upvector(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "upvector".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upvector_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvector".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dst_oct(mut self, val: i32) -> Self {
        self.params.insert(
            "dst_oct".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_dst_oct_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dst_oct".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dst_offsettype(mut self, val: VopUndulatusDstOffsettype) -> Self {
        self.params.insert(
            "dst_offsettype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dst_offsettype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dst_offsettype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mirror(mut self, val: bool) -> Self {
        self.params.insert(
            "mirror".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mirror_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mirror".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_distortwaves(mut self, val: bool) -> Self {
        self.params.insert(
            "distortwaves".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_distortwaves_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distortwaves".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dst_dodroop(mut self, val: bool) -> Self {
        self.params.insert(
            "dst_dodroop".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dst_dodroop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dst_dodroop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopUndulatus {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "undulatus"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUndulatusOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "noise"
    fn out_noise(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("noise".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopUndulatusOutputs for VopUndulatus {}
impl VopUndulatusOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopUndulatus> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopUndulatusInnerExt {
    fn add4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn align1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn blur(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cloudnoise1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn complement1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn complement2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn constant_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn distort(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn dodroop(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn droop(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn droopdir(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fit1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fit2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn floattovec1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn floattovec4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn freq(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn input3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn invert1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn invert3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn lac2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn makexform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mirror(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn next(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn next1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn next2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn oct(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn offset1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn offset2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm13(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ramps1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rough(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn snippet2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn snippet3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn srcmin(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn srcmin1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn srcmin3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn stretch(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn vecsetcompon1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopUndulatusInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopUndulatus> {
    fn add4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add4")
    }
    fn align1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("align1")
    }
    fn blur(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("blur")
    }
    fn cloudnoise1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("cloudnoise1")
    }
    fn complement1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("complement1")
    }
    fn complement2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("complement2")
    }
    fn constant_3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("constant_3")
    }
    fn distort(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("distort")
    }
    fn dodroop(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("dodroop")
    }
    fn droop(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("droop")
    }
    fn droopdir(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("droopdir")
    }
    fn fit1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fit1")
    }
    fn fit2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fit2")
    }
    fn floattovec1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("floattovec1")
    }
    fn floattovec4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("floattovec4")
    }
    fn freq(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("freq")
    }
    fn input3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("input3")
    }
    fn invert1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("invert1")
    }
    fn invert3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("invert3")
    }
    fn lac2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("lac2")
    }
    fn makexform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("makexform1")
    }
    fn mirror(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("mirror")
    }
    fn multiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply1")
    }
    fn multiply4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply4")
    }
    fn next(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("next")
    }
    fn next1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("next1")
    }
    fn next2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("next2")
    }
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null1")
    }
    fn null2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null2")
    }
    fn oct(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("oct")
    }
    fn offset1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("offset1")
    }
    fn offset2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("offset2")
    }
    fn parm1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm1")
    }
    fn parm13(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm13")
    }
    fn parm2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm2")
    }
    fn parm3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm3")
    }
    fn ramps1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ramps1")
    }
    fn rough(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rough")
    }
    fn snippet2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("snippet2")
    }
    fn snippet3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("snippet3")
    }
    fn srcmin(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("srcmin")
    }
    fn srcmin1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("srcmin1")
    }
    fn srcmin3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("srcmin3")
    }
    fn stretch(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("stretch")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn switch5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch5")
    }
    fn vecsetcompon1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("vecsetcompon1")
    }
}

#[derive(Debug, Clone)]
pub struct VopUnifiednoise {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUnifiednoise {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_sample_location_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_pos<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("pos".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_noise_type_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_basis<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("basis".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_freq<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("freq".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_offset_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_offset<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("offset".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_period_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_period<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("period".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_fractal_type_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fractal<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fractal".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_max_octaves_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_oct<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("oct".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_lacunarity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_lac<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("lac".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_lattice_warp_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dolwarp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dolwarp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_accumulate_lattice_warp_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_accuml<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("accuml".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_lattice_warp_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_disp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("disp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_freq_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(11),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dispfreq<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dispfreq".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_gradient_warp_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(12),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dogwarp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dogwarp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_accumulate_gradient_warp_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(13),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_accumg<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("accumg".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_gradient_warp_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(14),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_gflow<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("gflow".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_flow_rotation_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(15),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_flowrot<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("flowrot".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(16),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rough<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rough".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_output_correction_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(17),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_docc<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("docc".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_fold_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(18),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_fold<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_fold".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_bias_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(19),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_dobias<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_dobias".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_bias_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(20),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_bias<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_bias".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_gain_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(21),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_dogain<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_dogain".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_gain_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(22),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_gain<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_gain".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_complement_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(23),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_inv<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_inv".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_output_range_clamped_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(24),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_dorng<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_dorng".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_new_minimum_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(25),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_rnglo<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_rnglo".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_new_maximum_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(26),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_rnghi<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_rnghi".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_final_amplitude_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(27),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_amp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_amp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_filter_width_override_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(28),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fw<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fw".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_filter_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(29),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fscale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fscale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_signature_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(30),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_signature<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("signature".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_periodic_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(31),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_periodic<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("periodic".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_oct(mut self, val: f32) -> Self {
        self.params.insert(
            "oct".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oct_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oct".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lac(mut self, val: f32) -> Self {
        self.params.insert(
            "lac".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lac_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lac".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "rough".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rough".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_disp(mut self, val: f32) -> Self {
        self.params.insert(
            "disp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_disp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "disp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dispfreq(mut self, val: f32) -> Self {
        self.params.insert(
            "dispfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dispfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_gflow(mut self, val: f32) -> Self {
        self.params.insert(
            "gflow".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gflow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gflow".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flowrot(mut self, val: f32) -> Self {
        self.params.insert(
            "flowrot".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_flowrot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flowrot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fscale(mut self, val: f32) -> Self {
        self.params.insert(
            "fscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_bias(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cc_bias".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cc_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_bias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_gain(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cc_gain".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cc_gain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_gain".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_rnglo(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cc_rnglo".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cc_rnglo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_rnglo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_rnghi(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cc_rnghi".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cc_rnghi_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_rnghi".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_amp(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cc_amp".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cc_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_amp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_freq(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_freq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_period(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "period".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_period_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "period".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basis(mut self, val: &str) -> Self {
        self.params.insert(
            "basis".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fractal(mut self, val: &str) -> Self {
        self.params.insert(
            "fractal".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fractal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fractal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_periodic(mut self, val: bool) -> Self {
        self.params.insert(
            "periodic".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_periodic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "periodic".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dolwarp(mut self, val: bool) -> Self {
        self.params.insert(
            "dolwarp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dolwarp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dolwarp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_accuml(mut self, val: bool) -> Self {
        self.params.insert(
            "accuml".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_accuml_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accuml".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dogwarp(mut self, val: bool) -> Self {
        self.params.insert(
            "dogwarp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dogwarp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dogwarp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_accumg(mut self, val: bool) -> Self {
        self.params.insert(
            "accumg".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_accumg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accumg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_fold(mut self, val: bool) -> Self {
        self.params.insert(
            "cc_fold".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cc_fold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_fold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_dobias(mut self, val: bool) -> Self {
        self.params.insert(
            "cc_dobias".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cc_dobias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_dobias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_dogain(mut self, val: bool) -> Self {
        self.params.insert(
            "cc_dogain".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cc_dogain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_dogain".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_inv(mut self, val: bool) -> Self {
        self.params.insert(
            "cc_inv".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cc_inv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_inv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_dorng(mut self, val: bool) -> Self {
        self.params.insert(
            "cc_dorng".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cc_dorng_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_dorng".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopUnifiednoise {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "unifiednoise"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUnifiednoiseOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Noise"
    fn out_noise(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("noise".to_string()),
        }
    }
    /// Output pin: "Median For Current Parameterization"
    fn out_x_avg(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("x_avg".to_string()),
        }
    }
    /// Output pin: "Actual Number Of Octaves"
    fn out_x_oct(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("x_oct".to_string()),
        }
    }
    /// Output pin: "Offset Due To Warps (Current Space)"
    fn out_x_off(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("x_off".to_string()),
        }
    }
}

impl VopUnifiednoiseOutputs for VopUnifiednoise {}
impl VopUnifiednoiseOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopUnifiednoise> {}

#[derive(Debug, Clone)]
pub struct VopUnifiednoiseStatic {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUnifiednoiseStatic {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_sample_location_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_pos<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("pos".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_freq<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("freq".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_offset_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_offset<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("offset".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_period_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_period<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("period".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_max_octaves_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_oct<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("oct".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_lacunarity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_lac<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("lac".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_lattice_warp_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dolwarp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dolwarp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_accumulate_lattice_warp_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_accuml<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("accuml".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_lattice_warp_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_disp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("disp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_freq_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dispfreq<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dispfreq".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_enable_gradient_warp_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_dogwarp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("dogwarp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_accumulate_gradient_warp_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(11),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_accumg<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("accumg".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_gradient_warp_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(12),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_gflow<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("gflow".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_flow_rotation_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(13),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_flowrot<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("flowrot".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(14),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rough<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rough".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_output_correction_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(15),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_docc<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("docc".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_fold_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(16),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_fold<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_fold".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_do_bias_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(17),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_dobias<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_dobias".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_bias_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(18),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_bias<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_bias".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_do_gain_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(19),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_dogain<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_dogain".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_gain_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(20),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_gain<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_gain".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_complement_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(21),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_inv<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_inv".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_output_range_clamped_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(22),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_dorng<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_dorng".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_new_minimum_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(23),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_rnglo<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_rnglo".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_new_maximum_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(24),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_rnghi<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_rnghi".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_final_amplitude_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(25),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_cc_amp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("cc_amp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_filter_width_override_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(26),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fw<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fw".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_filter_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(27),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fscale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fscale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_oct(mut self, val: f32) -> Self {
        self.params.insert(
            "oct".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oct_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oct".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lac(mut self, val: f32) -> Self {
        self.params.insert(
            "lac".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lac_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lac".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "rough".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rough".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_disp(mut self, val: f32) -> Self {
        self.params.insert(
            "disp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_disp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "disp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dispfreq(mut self, val: f32) -> Self {
        self.params.insert(
            "dispfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dispfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_gflow(mut self, val: f32) -> Self {
        self.params.insert(
            "gflow".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gflow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gflow".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flowrot(mut self, val: f32) -> Self {
        self.params.insert(
            "flowrot".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_flowrot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flowrot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fscale(mut self, val: f32) -> Self {
        self.params.insert(
            "fscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_bias(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cc_bias".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cc_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_bias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_gain(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cc_gain".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cc_gain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_gain".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_rnglo(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cc_rnglo".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cc_rnglo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_rnglo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_rnghi(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cc_rnghi".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cc_rnghi_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_rnghi".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_amp(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cc_amp".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cc_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_amp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_freq(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_freq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_period(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "period".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_period_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "period".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basis(mut self, val: &str) -> Self {
        self.params.insert(
            "basis".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fractal(mut self, val: &str) -> Self {
        self.params.insert(
            "fractal".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fractal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fractal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_periodic(mut self, val: bool) -> Self {
        self.params.insert(
            "periodic".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_periodic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "periodic".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dolwarp(mut self, val: bool) -> Self {
        self.params.insert(
            "dolwarp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dolwarp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dolwarp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_accuml(mut self, val: bool) -> Self {
        self.params.insert(
            "accuml".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_accuml_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accuml".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dogwarp(mut self, val: bool) -> Self {
        self.params.insert(
            "dogwarp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dogwarp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dogwarp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_accumg(mut self, val: bool) -> Self {
        self.params.insert(
            "accumg".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_accumg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accumg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_fold(mut self, val: bool) -> Self {
        self.params.insert(
            "cc_fold".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cc_fold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_fold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_dobias(mut self, val: bool) -> Self {
        self.params.insert(
            "cc_dobias".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cc_dobias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_dobias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_dogain(mut self, val: bool) -> Self {
        self.params.insert(
            "cc_dogain".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cc_dogain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_dogain".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_inv(mut self, val: bool) -> Self {
        self.params.insert(
            "cc_inv".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cc_inv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_inv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cc_dorng(mut self, val: bool) -> Self {
        self.params.insert(
            "cc_dorng".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cc_dorng_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cc_dorng".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopUnifiednoiseStatic {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "unifiednoise_static"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUnifiednoiseStaticOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Noise"
    fn out_noise(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("noise".to_string()),
        }
    }
    /// Output pin: "Median For Current Parameterization"
    fn out_x_avg(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("x_avg".to_string()),
        }
    }
    /// Output pin: "Actual Number Of Octaves"
    fn out_x_oct(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("x_oct".to_string()),
        }
    }
    /// Output pin: "Offset Due To Warps (Current Space)"
    fn out_x_off(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("x_off".to_string()),
        }
    }
}

impl VopUnifiednoiseStaticOutputs for VopUnifiednoiseStatic {}
impl VopUnifiednoiseStaticOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopUnifiednoiseStatic>
{
}

#[derive(Debug, Clone)]
pub struct VopUniqueval {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUniqueval {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_geometry_file_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_filename<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("filename".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_attribute_class_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_atype<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("atype".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_attribute_name_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_attrib<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("attrib".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_index_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_index<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("index".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_index(mut self, val: i32) -> Self {
        self.params.insert(
            "index".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_index_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "index".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
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
    pub fn with_atype(mut self, val: &str) -> Self {
        self.params.insert(
            "atype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_atype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "atype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attrib(mut self, val: &str) -> Self {
        self.params.insert(
            "attrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopUniqueval {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "uniqueval"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUniquevalOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Unique Value of Attribute"
    fn out_val(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("val".to_string()),
        }
    }
}

impl VopUniquevalOutputs for VopUniqueval {}
impl VopUniquevalOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopUniqueval> {}

#[derive(Debug, Clone)]
pub struct VopUsdglobal {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUsdglobal {
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

impl houdini_ramen_core::types::HoudiniNode for VopUsdglobal {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "usdglobal"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUsdglobalOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Element Number"
    fn out_elemnum(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("elemnum".to_string()),
        }
    }
    /// Output pin: "Element Number"
    fn out_numelem(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("numelem".to_string()),
        }
    }
    /// Output pin: "Primitive Path"
    fn out_primpath(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("primpath".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopUsdglobalOutputs for VopUsdglobal {}
impl VopUsdglobalOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopUsdglobal> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopUsdglobalInnerExt {
    fn parm1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopUsdglobalInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopUsdglobal> {
    fn parm1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm1")
    }
    fn parm2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm2")
    }
    fn parm3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm3")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
}

#[derive(Debug, Clone)]
pub struct VopUsdpreviewsurface {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUsdpreviewsurface {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_diffuse_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_diffusecolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("diffuseColor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_emissive_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_emissivecolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("emissiveColor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_use_specular_workflow_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_usespecularworkflow<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("useSpecularWorkflow".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_specular_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_specularcolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("specularColor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_metallic_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_metallic<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("metallic".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_roughness<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("roughness".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_clearcoat_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_clearcoat<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("clearcoat".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_clearcoat_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_clearcoatroughness<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("clearcoatRoughness".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_opacity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_opacity<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("opacity".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_index_of_refraction_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_ior<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("ior".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_normal<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("normal".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_displacement_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(11),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_displacement<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("displacement".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_occlusion_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(12),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_occlusion<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("occlusion".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_metallic(mut self, val: f32) -> Self {
        self.params.insert(
            "metallic".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_metallic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "metallic".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_roughness(mut self, val: f32) -> Self {
        self.params.insert(
            "roughness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roughness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roughness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clearcoat(mut self, val: f32) -> Self {
        self.params.insert(
            "clearcoat".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clearcoat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clearcoat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clearcoatroughness(mut self, val: f32) -> Self {
        self.params.insert(
            "clearcoatRoughness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clearcoatroughness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clearcoatRoughness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_opacity(mut self, val: f32) -> Self {
        self.params.insert(
            "opacity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_opacity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_opacitythreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "opacityThreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_opacitythreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacityThreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ior(mut self, val: f32) -> Self {
        self.params.insert(
            "ior".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ior".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_occlusion(mut self, val: f32) -> Self {
        self.params.insert(
            "occlusion".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_occlusion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "occlusion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displacement(mut self, val: f32) -> Self {
        self.params.insert(
            "displacement".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_displacement_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displacement".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_diffusecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "diffuseColor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_diffusecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseColor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_emissivecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "emissiveColor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_emissivecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emissiveColor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specularcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "specularColor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_specularcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specularColor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_normal(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "normal".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_normal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usespecularworkflow(mut self, val: bool) -> Self {
        self.params.insert(
            "useSpecularWorkflow".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usespecularworkflow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useSpecularWorkflow".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopUsdpreviewsurface {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "usdpreviewsurface"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUsdpreviewsurfaceOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Surface"
    fn out_surface(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("surface".to_string()),
        }
    }
    /// Output pin: "Displacement"
    fn out_displacement(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("displacement".to_string()),
        }
    }
}

impl VopUsdpreviewsurfaceOutputs for VopUsdpreviewsurface {}
impl VopUsdpreviewsurfaceOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopUsdpreviewsurface>
{
}

#[derive(Debug, Clone)]
pub struct VopUsdprimvarreader {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUsdprimvarreader {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_var_name_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_varname<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("varname".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_fallback_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fallback<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fallback".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_fallback(mut self, val: f32) -> Self {
        self.params.insert(
            "fallback".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fallback_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fallback".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fallback_float2(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "fallback_float2".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_fallback_float2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fallback_float2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fallback_float3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fallback_float3".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fallback_float3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fallback_float3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fallback_normal(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fallback_normal".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fallback_normal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fallback_normal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fallback_point(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fallback_point".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fallback_point_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fallback_point".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fallback_vector(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fallback_vector".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fallback_vector_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fallback_vector".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fallback_float4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "fallback_float4".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_fallback_float4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fallback_float4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fallback_matrix(mut self, val: Vec<f32>) -> Self {
        self.params.insert(
            "fallback_matrix".to_string(),
            houdini_ramen_core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_fallback_matrix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fallback_matrix".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fallback_int(mut self, val: i32) -> Self {
        self.params.insert(
            "fallback_int".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_fallback_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fallback_int".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_varname(mut self, val: &str) -> Self {
        self.params.insert(
            "varname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_varname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "varname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fallback_string(mut self, val: &str) -> Self {
        self.params.insert(
            "fallback_string".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fallback_string_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fallback_string".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopUsdprimvarreader {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "usdprimvarreader"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUsdprimvarreaderOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Value"
    fn out_result(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("result".to_string()),
        }
    }
}

impl VopUsdprimvarreaderOutputs for VopUsdprimvarreader {}
impl VopUsdprimvarreaderOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopUsdprimvarreader>
{
}

#[derive(Debug, Clone)]
pub struct VopUsdtransform2d {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUsdtransform2d {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_in<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("in".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_rotation(mut self, val: f32) -> Self {
        self.params.insert(
            "rotation".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_in(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "in".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_in_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "in".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_translation(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "translation".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_translation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "translation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scale(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
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
}

impl houdini_ramen_core::types::HoudiniNode for VopUsdtransform2d {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "usdtransform2d"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUsdtransform2dOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Result"
    fn out_result(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("result".to_string()),
        }
    }
}

impl VopUsdtransform2dOutputs for VopUsdtransform2d {}
impl VopUsdtransform2dOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopUsdtransform2d>
{
}

#[derive(Debug, Clone)]
pub struct VopUsduvtexture {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUsduvtexture {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_file_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_file<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("file".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_uv_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_st<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("st".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_wrap_s_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_wraps<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("wrapS".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_wrap_t_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_wrapt<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("wrapT".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_fallback_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_fallback<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("fallback".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_scale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("scale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_bias_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_bias<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("bias".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_source_color_space_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_sourcecolorspace<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("sourceColorSpace".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_st(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "st".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_st_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "st".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fallback(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "fallback".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_fallback_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fallback".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scale(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
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
    pub fn with_bias(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "bias".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_file(mut self, val: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wraps(mut self, val: &str) -> Self {
        self.params.insert(
            "wrapS".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wraps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wrapS".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wrapt(mut self, val: &str) -> Self {
        self.params.insert(
            "wrapT".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wrapt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wrapT".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "sourceColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopUsduvtexture {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "usduvtexture"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUsduvtextureOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "R"
    fn out_r(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("r".to_string()),
        }
    }
    /// Output pin: "G"
    fn out_g(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("g".to_string()),
        }
    }
    /// Output pin: "B"
    fn out_b(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("b".to_string()),
        }
    }
    /// Output pin: "A"
    fn out_a(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("a".to_string()),
        }
    }
    /// Output pin: "RGB"
    fn out_rgb(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("rgb".to_string()),
        }
    }
}

impl VopUsduvtextureOutputs for VopUsduvtexture {}
impl VopUsduvtextureOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopUsduvtexture> {}

#[derive(Debug, Clone)]
pub struct VopUvcoords {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUvcoords {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_mode_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_mode<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("mode".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_mode(mut self, val: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
    pub fn with_uvattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "uvattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uvattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopUvcoords {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "uvcoords"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUvcoordsOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Chosen Value"
    fn out_uv(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("uv".to_string()),
        }
    }
    /// Output pin: "Component 1"
    fn out_u(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("u".to_string()),
        }
    }
    /// Output pin: "Component 2"
    fn out_v(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("v".to_string()),
        }
    }
    /// Output pin: "Component 3"
    fn out_w(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("w".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopUvcoordsOutputs for VopUvcoords {}
impl VopUvcoordsOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopUvcoords> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopUvcoordsInnerExt {
    fn compare1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn s_t(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn st_mode(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn vectofloat1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopUvcoordsInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopUvcoords> {
    fn compare1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("compare1")
    }
    fn parm1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm1")
    }
    fn parm2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm2")
    }
    fn s_t(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("s_t")
    }
    fn st_mode(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("st_mode")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn switch1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch1")
    }
    fn switch2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch2")
    }
    fn vectofloat1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("vectofloat1")
    }
}

#[derive(Debug, Clone)]
pub struct VopUvnoise {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUvnoise {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_uv_coordinates_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_uv<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("uv".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_freq<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("freq".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_offset_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_offset<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("offset".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_amplitude_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_amp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("amp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rough<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rough".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_clamp_to_min_max_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_clamp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("clamp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_clamp_min_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_min<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("min".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_clamp_max_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_max<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("max".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_clamp_rolloff_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rolloff<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rolloff".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_compute_absolute_noise_value_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_absolute<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("absolute".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_amp(mut self, val: f32) -> Self {
        self.params.insert(
            "amp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "rough".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rough".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_min(mut self, val: f32) -> Self {
        self.params.insert(
            "min".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "min".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_max(mut self, val: f32) -> Self {
        self.params.insert(
            "max".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_max_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "max".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rolloff(mut self, val: f32) -> Self {
        self.params.insert(
            "rolloff".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rolloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rolloff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_freq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_freq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clamp(mut self, val: bool) -> Self {
        self.params.insert(
            "clamp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clamp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_absolute(mut self, val: bool) -> Self {
        self.params.insert(
            "absolute".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_absolute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "absolute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopUvnoise {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "uvnoise"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUvnoiseOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Noisy S Coordinate"
    fn out_noiseuv(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("noiseuv".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopUvnoiseOutputs for VopUvnoise {}
impl VopUvnoiseOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopUvnoise> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopUvnoiseInnerExt {
    fn aanoise1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn absolute_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn absolute_test(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn add1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clamp_snoise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clamp_snoise1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clamp_snoise2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clamptest_s(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn floattovec1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn s_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn st_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn uvcoords1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopUvnoiseInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopUvnoise> {
    fn aanoise1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("aanoise1")
    }
    fn absolute_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("absolute_noise")
    }
    fn absolute_test(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("absolute_test")
    }
    fn add1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add1")
    }
    fn clamp_snoise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clamp_snoise")
    }
    fn clamp_snoise1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clamp_snoise1")
    }
    fn clamp_snoise2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clamp_snoise2")
    }
    fn clamptest_s(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clamptest_s")
    }
    fn floattovec1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("floattovec1")
    }
    fn s_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("s_input")
    }
    fn st_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("st_noise")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn uvcoords1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("uvcoords1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopUvplanarprojectAxisswitch {
    /// X-axis
    XMinusAxis = 0,
    /// Y-axis
    YMinusAxis = 1,
    /// Z-axis
    ZMinusAxis = 2,
}

#[derive(Debug, Clone)]
pub struct VopUvplanarproject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUvplanarproject {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_surface_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_p<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("P".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_surface_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_n<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("N".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_scale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("scale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_angle_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_angle<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("angle".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_axis_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_axisswitch<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("axisswitch".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_mask_sharpness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_masksharp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("masksharp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_scale_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_negscale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("negscale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_angle_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_negangle<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("negangle".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_separate_axis_controls_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_sepaxis<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("sepaxis".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_uv_offset_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_uvoffset<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("uvoffset".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_uvoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "uvoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uvoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_masksharp(mut self, val: f32) -> Self {
        self.params.insert(
            "masksharp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_masksharp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "masksharp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_angle(mut self, val: f32) -> Self {
        self.params.insert(
            "angle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_negangle(mut self, val: f32) -> Self {
        self.params.insert(
            "negangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_negangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "negangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "P".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "P".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_n(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "N".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_n_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "N".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
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
    pub fn with_negscale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "negscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_negscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "negscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axisswitch(mut self, val: VopUvplanarprojectAxisswitch) -> Self {
        self.params.insert(
            "axisswitch".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_axisswitch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axisswitch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sepaxis(mut self, val: bool) -> Self {
        self.params.insert(
            "sepaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sepaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sepaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopUvplanarproject {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "uvplanarproject"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUvplanarprojectOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Positive Axis UV"
    fn out_pos_uv(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("pos_uv".to_string()),
        }
    }
    /// Output pin: "Negative Axis UV"
    fn out_neg_uv(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("neg_uv".to_string()),
        }
    }
    /// Output pin: "Positive Axis Mask"
    fn out_pos_mask(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("pos_mask".to_string()),
        }
    }
    /// Output pin: "Negative Axis Mask"
    fn out_neg_mask(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("neg_mask".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopUvplanarprojectOutputs for VopUvplanarproject {}
impl VopUvplanarprojectOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopUvplanarproject>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopUvplanarprojectInnerExt {
    fn isolatex(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn isolatey(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn isolatez(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn projectonx(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn projectonx2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn projectony(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn projectony2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn projectonz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn projectonz2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn add1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn add2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clamp2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clamp3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn const1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn degtorad1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn degtorad2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn dot2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn dot3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fit1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fit2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn fit3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn floattovec1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn floattovec2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn global1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn global2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ifconnected1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ifconnected2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn negate1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn normalize1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pow2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pow3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rotate1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rotate2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn scale1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn scale2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn srcmin(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subconst1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switcher(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switcher1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switcher2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn transform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn transform2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn transform4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopUvplanarprojectInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, VopUvplanarproject>
{
    fn isolatex(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("IsolateX")
    }
    fn isolatey(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("IsolateY")
    }
    fn isolatez(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("IsolateZ")
    }
    fn projectonx(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ProjectOnX")
    }
    fn projectonx2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ProjectOnX2")
    }
    fn projectony(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ProjectOnY")
    }
    fn projectony2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ProjectOnY2")
    }
    fn projectonz(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ProjectOnZ")
    }
    fn projectonz2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ProjectOnZ2")
    }
    fn add1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add1")
    }
    fn add2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add2")
    }
    fn clamp2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clamp2")
    }
    fn clamp3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clamp3")
    }
    fn const1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("const1")
    }
    fn degtorad1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("degtorad1")
    }
    fn degtorad2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("degtorad2")
    }
    fn dot2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("dot2")
    }
    fn dot3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("dot3")
    }
    fn fit1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fit1")
    }
    fn fit2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fit2")
    }
    fn fit3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("fit3")
    }
    fn floattovec1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("floattovec1")
    }
    fn floattovec2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("floattovec2")
    }
    fn global1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("global1")
    }
    fn global2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("global2")
    }
    fn ifconnected1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ifconnected1")
    }
    fn ifconnected2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ifconnected2")
    }
    fn multiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply1")
    }
    fn multiply3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply3")
    }
    fn multiply4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply4")
    }
    fn multiply5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply5")
    }
    fn negate1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("negate1")
    }
    fn normalize1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("normalize1")
    }
    fn parm1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm1")
    }
    fn parm2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm2")
    }
    fn parm3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm3")
    }
    fn parm4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm4")
    }
    fn parm5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm5")
    }
    fn parm6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm6")
    }
    fn pow2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pow2")
    }
    fn pow3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pow3")
    }
    fn rotate1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rotate1")
    }
    fn rotate2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rotate2")
    }
    fn scale1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("scale1")
    }
    fn scale2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("scale2")
    }
    fn srcmin(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("srcmin")
    }
    fn subconst1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subconst1")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn switch1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch1")
    }
    fn switch2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch2")
    }
    fn switch3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch3")
    }
    fn switch4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch4")
    }
    fn switch5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch5")
    }
    fn switch6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch6")
    }
    fn switch7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch7")
    }
    fn switch8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch8")
    }
    fn switcher(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switcher")
    }
    fn switcher1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switcher1")
    }
    fn switcher2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switcher2")
    }
    fn transform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("transform1")
    }
    fn transform2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("transform2")
    }
    fn transform4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("transform4")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopUvpositionTrs {
    ScaleRotateTranslate = 0,
    ScaleTranslateRotate = 1,
    RotateScaleTranslate = 2,
    RotateTranslateScale = 3,
    TranslateScaleRotate = 4,
    TranslateRotateScale = 5,
}

#[derive(Debug, Clone)]
pub struct VopUvposition {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUvposition {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_chosen_value_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_uv<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("uv".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transform_order_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trs<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trs".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_translate_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trans<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trans".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_scale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("scale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_rotate_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rot<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rot".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_pivot_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_pivot<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("pivot".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_rot(mut self, val: f32) -> Self {
        self.params.insert(
            "rot".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trans(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "trans".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_trans_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trans".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scale(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
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
    pub fn with_pivot(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "pivot".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pivot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pivot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trs(mut self, val: VopUvpositionTrs) -> Self {
        self.params.insert(
            "trs".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_trs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopUvposition {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "uvposition"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUvpositionOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Combined Value"
    fn out_posuv(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("posuv".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopUvpositionOutputs for VopUvposition {}
impl VopUvpositionOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopUvposition> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopUvpositionInnerExt {
    fn floattovec1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ifconnected1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn invert1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn makexform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pivot(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn rot(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn scale(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn trans(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn trs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn uvcoords1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopUvpositionInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopUvposition> {
    fn floattovec1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("floattovec1")
    }
    fn ifconnected1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ifconnected1")
    }
    fn invert1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("invert1")
    }
    fn makexform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("makexform1")
    }
    fn multiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply1")
    }
    fn pivot(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pivot")
    }
    fn rot(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("rot")
    }
    fn scale(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("scale")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn trans(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("trans")
    }
    fn trs(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("trs")
    }
    fn uvcoords1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("uvcoords1")
    }
}

#[derive(Debug, Clone)]
pub struct VopUvproject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUvproject {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_projection_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_proj<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("proj".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_p<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("P".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_scale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("scale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_offset_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_offset<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("offset".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_scale(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
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
    pub fn with_offset(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proj(mut self, val: &str) -> Self {
        self.params.insert(
            "proj".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proj_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "proj".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopUvproject {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "uvproject"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUvprojectOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "UV Coordinates"
    fn out_uv(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("uv".to_string()),
        }
    }
    /// Output pin: "UV Derivative"
    fn out_duv(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("duv".to_string()),
        }
    }
}

impl VopUvprojectOutputs for VopUvproject {}
impl VopUvprojectOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopUvproject> {}

#[derive(Debug, Clone)]
pub struct VopUvspacechg {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUvspacechg {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_uvw_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_p<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("P".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_space_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_space<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("space".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_space(mut self, val: &str) -> Self {
        self.params.insert(
            "space".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_space_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopUvspacechg {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "uvspacechg"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUvspacechgOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Transformed UVW Position"
    fn out_xformp(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("xformP".to_string()),
        }
    }
}

impl VopUvspacechgOutputs for VopUvspacechg {}
impl VopUvspacechgOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopUvspacechg> {}

#[derive(Debug, Clone)]
pub struct VopUvtriplanarproject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUvtriplanarproject {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_surface_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_p<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("P".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_surface_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_n<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("N".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_xaxiscolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("xaxiscolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_color_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_negxaxiscolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("negxaxiscolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_color_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_zaxiscolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("zaxiscolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_color_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_negzaxiscolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("negzaxiscolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_color_4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_yaxiscolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("yaxiscolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_color_5_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_negyaxiscolor<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("negyaxiscolor".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tint_with_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_xtint<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("xtint".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tint_with_color_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_ztint<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("ztint".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tint_with_color_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_ytint<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("ytint".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_separate_axis_controls_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(11),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_xsep<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("xsep".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_separate_axis_controls_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(12),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_zsep<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("zsep".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_separate_axis_controls_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(13),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_ysep<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("ysep".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_mask_sharpness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(14),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_xmasksharp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("xmasksharp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_mask_sharpness_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(15),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_zmasksharp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("zmasksharp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_mask_sharpness_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(16),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_ymasksharp<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("ymasksharp".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(17),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_xscale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("xscale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_angle_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(18),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_xangle<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("xangle".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_scale_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(19),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_xnegscale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("xnegscale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_angle_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(20),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_xnegangle<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("xnegangle".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tint_with_color_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(21),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_xnegtint<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("xnegtint".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tint_with_color_4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(22),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_znegtint<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("znegtint".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_angle_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(23),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_zangle<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("zangle".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_scale_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(24),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_znegscale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("znegscale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_angle_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(25),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_znegangle<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("znegangle".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_scale_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(26),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_zscale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("zscale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_tint_with_color_5_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(27),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_ynegtint<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("ynegtint".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_scale_4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(28),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_yscale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("yscale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_angle_4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(29),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_yangle<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("yangle".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_scale_5_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(30),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_ynegscale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("ynegscale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_angle_5_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(31),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_ynegangle<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("ynegangle".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_texture_map_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(32),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_xnegmap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("xnegmap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_texture_map_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(33),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_xposmap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("xposmap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_texture_map_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(34),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_znegmap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("znegmap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_texture_map_3_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(35),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_zposmap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("zposmap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_texture_map_4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(36),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_ynegmap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("ynegmap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_texture_map_5_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(37),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_yposmap<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("yposmap".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_axis_offset_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(38),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_axisoffset<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("axisoffset".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_xmasksharp(mut self, val: f32) -> Self {
        self.params.insert(
            "xmasksharp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xmasksharp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xmasksharp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xangle(mut self, val: f32) -> Self {
        self.params.insert(
            "xangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xnegangle(mut self, val: f32) -> Self {
        self.params.insert(
            "xnegangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xnegangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xnegangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ymasksharp(mut self, val: f32) -> Self {
        self.params.insert(
            "ymasksharp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ymasksharp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ymasksharp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_yangle(mut self, val: f32) -> Self {
        self.params.insert(
            "yangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_yangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ynegangle(mut self, val: f32) -> Self {
        self.params.insert(
            "ynegangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ynegangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ynegangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_zmasksharp(mut self, val: f32) -> Self {
        self.params.insert(
            "zmasksharp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zmasksharp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zmasksharp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_zangle(mut self, val: f32) -> Self {
        self.params.insert(
            "zangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_znegangle(mut self, val: f32) -> Self {
        self.params.insert(
            "znegangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_znegangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "znegangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "P".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "P".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_n(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "N".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_n_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "N".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axisoffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "axisoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_axisoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axisoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xaxiscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xaxiscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xaxiscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xscale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_negxaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "negxaxiscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_negxaxiscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "negxaxiscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xnegscale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xnegscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xnegscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xnegscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_yaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "yaxiscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_yaxiscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yaxiscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_yscale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "yscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_yscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_negyaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "negyaxiscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_negyaxiscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "negyaxiscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ynegscale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ynegscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ynegscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ynegscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_zaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "zaxiscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_zaxiscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zaxiscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_zscale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "zscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_zscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_negzaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "negzaxiscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_negzaxiscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "negzaxiscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_znegscale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "znegscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_znegscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "znegscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xposmap(mut self, val: &str) -> Self {
        self.params.insert(
            "xposmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xposmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xposmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xnegmap(mut self, val: &str) -> Self {
        self.params.insert(
            "xnegmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xnegmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xnegmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_yposmap(mut self, val: &str) -> Self {
        self.params.insert(
            "yposmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_yposmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yposmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ynegmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ynegmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ynegmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ynegmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_zposmap(mut self, val: &str) -> Self {
        self.params.insert(
            "zposmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_zposmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zposmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_znegmap(mut self, val: &str) -> Self {
        self.params.insert(
            "znegmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_znegmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "znegmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xsep(mut self, val: bool) -> Self {
        self.params.insert(
            "xsep".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xsep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xsep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xtint(mut self, val: bool) -> Self {
        self.params.insert(
            "xtint".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xtint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xtint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xnegtint(mut self, val: bool) -> Self {
        self.params.insert(
            "xnegtint".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_xnegtint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xnegtint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ysep(mut self, val: bool) -> Self {
        self.params.insert(
            "ysep".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ysep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ysep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ytint(mut self, val: bool) -> Self {
        self.params.insert(
            "ytint".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ytint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ytint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ynegtint(mut self, val: bool) -> Self {
        self.params.insert(
            "ynegtint".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ynegtint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ynegtint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_zsep(mut self, val: bool) -> Self {
        self.params.insert(
            "zsep".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_zsep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zsep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ztint(mut self, val: bool) -> Self {
        self.params.insert(
            "ztint".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ztint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ztint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_znegtint(mut self, val: bool) -> Self {
        self.params.insert(
            "znegtint".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_znegtint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "znegtint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopUvtriplanarproject {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "uvtriplanarproject"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUvtriplanarprojectOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Blended Color"
    fn out_clr(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("clr".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopUvtriplanarprojectOutputs for VopUvtriplanarproject {}
impl VopUvtriplanarprojectOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<VopUvtriplanarproject>
{
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopUvtriplanarprojectInnerExt {
    fn and1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn and2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn and3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn and4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn and5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn and6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn angle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn angle2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn angle3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn colormix1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn colormix2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn colormix3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn colormix4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn colormix5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn complement1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn complement2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn complement3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn global1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn ifconnected1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn map(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn map1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn map2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn map3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn map4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn map5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn masksharp(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn masksharp1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn masksharp2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn max1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn max2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn negangle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn negangle1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn negangle2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn negscale(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn negscale2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn negscale3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn or1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn or2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn or3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm10(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm11(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm12(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm13(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm14(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm15(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm16(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn parm9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn scale(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn scale1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subtract1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch10(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch11(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch12(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn texture1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn texture2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn texture3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn texture4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn texture5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn texture6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn texture_scale(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn transform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn transform2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn uvplanarproject1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn uvplanarproject2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn uvplanarproject3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn vec2tofloat1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn vec2tofloat2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn vec2tofloat3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn vec2tofloat4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn vec2tofloat5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn vec2tofloat6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopUvtriplanarprojectInnerExt
    for houdini_ramen_core::graph::InnerGraph<'a, VopUvtriplanarproject>
{
    fn and1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("and1")
    }
    fn and2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("and2")
    }
    fn and3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("and3")
    }
    fn and4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("and4")
    }
    fn and5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("and5")
    }
    fn and6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("and6")
    }
    fn angle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("angle")
    }
    fn angle2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("angle2")
    }
    fn angle3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("angle3")
    }
    fn colormix1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("colormix1")
    }
    fn colormix2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("colormix2")
    }
    fn colormix3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("colormix3")
    }
    fn colormix4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("colormix4")
    }
    fn colormix5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("colormix5")
    }
    fn complement1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("complement1")
    }
    fn complement2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("complement2")
    }
    fn complement3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("complement3")
    }
    fn global1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("global1")
    }
    fn ifconnected1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ifconnected1")
    }
    fn map(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("map")
    }
    fn map1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("map1")
    }
    fn map2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("map2")
    }
    fn map3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("map3")
    }
    fn map4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("map4")
    }
    fn map5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("map5")
    }
    fn masksharp(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("masksharp")
    }
    fn masksharp1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("masksharp1")
    }
    fn masksharp2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("masksharp2")
    }
    fn max1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("max1")
    }
    fn max2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("max2")
    }
    fn multiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply1")
    }
    fn multiply2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply2")
    }
    fn multiply3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply3")
    }
    fn multiply4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply4")
    }
    fn multiply5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply5")
    }
    fn multiply6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply6")
    }
    fn multiply7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply7")
    }
    fn multiply8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply8")
    }
    fn negangle(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("negangle")
    }
    fn negangle1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("negangle1")
    }
    fn negangle2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("negangle2")
    }
    fn negscale(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("negscale")
    }
    fn negscale2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("negscale2")
    }
    fn negscale3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("negscale3")
    }
    fn or1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("or1")
    }
    fn or2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("or2")
    }
    fn or3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("or3")
    }
    fn parm1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm1")
    }
    fn parm10(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm10")
    }
    fn parm11(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm11")
    }
    fn parm12(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm12")
    }
    fn parm13(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm13")
    }
    fn parm14(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm14")
    }
    fn parm15(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm15")
    }
    fn parm16(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm16")
    }
    fn parm2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm2")
    }
    fn parm3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm3")
    }
    fn parm4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm4")
    }
    fn parm5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm5")
    }
    fn parm6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm6")
    }
    fn parm7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm7")
    }
    fn parm8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm8")
    }
    fn parm9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("parm9")
    }
    fn scale(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("scale")
    }
    fn scale1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("scale1")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn subtract1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subtract1")
    }
    fn switch1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch1")
    }
    fn switch10(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch10")
    }
    fn switch11(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch11")
    }
    fn switch12(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch12")
    }
    fn switch2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch2")
    }
    fn switch3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch3")
    }
    fn switch4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch4")
    }
    fn switch5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch5")
    }
    fn switch6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch6")
    }
    fn switch7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch7")
    }
    fn switch8(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch8")
    }
    fn switch9(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch9")
    }
    fn texture1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("texture1")
    }
    fn texture2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("texture2")
    }
    fn texture3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("texture3")
    }
    fn texture4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("texture4")
    }
    fn texture5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("texture5")
    }
    fn texture6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("texture6")
    }
    fn texture_scale(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("texture_scale")
    }
    fn transform1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("transform1")
    }
    fn transform2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("transform2")
    }
    fn uvplanarproject1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("uvplanarproject1")
    }
    fn uvplanarproject2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("uvplanarproject2")
    }
    fn uvplanarproject3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("uvplanarproject3")
    }
    fn vec2tofloat1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("vec2tofloat1")
    }
    fn vec2tofloat2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("vec2tofloat2")
    }
    fn vec2tofloat3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("vec2tofloat3")
    }
    fn vec2tofloat4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("vec2tofloat4")
    }
    fn vec2tofloat5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("vec2tofloat5")
    }
    fn vec2tofloat6(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("vec2tofloat6")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopUvxformTrs {
    ScaleRotateTranslate = 0,
    ScaleTranslateRotate = 1,
    RotateScaleTranslate = 2,
    RotateTranslateScale = 3,
    TranslateScaleRotate = 4,
    TranslateRotateScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopUvxformXyz {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct VopUvxform {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopUvxform {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_texture_coordinates_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_uvw<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("uvw".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_transform_order_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trs<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trs".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_rotation_order_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_xyz<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("xyz".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_translate_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_trans<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("trans".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_rotate_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_rot<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("rot".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_scale<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("scale".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_pivot_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_pivot<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("pivot".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_trans(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "trans".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_trans_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trans".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rot(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rot".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
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
    pub fn with_pivot(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pivot".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pivot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pivot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trs(mut self, val: VopUvxformTrs) -> Self {
        self.params.insert(
            "trs".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_trs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xyz(mut self, val: VopUvxformXyz) -> Self {
        self.params.insert(
            "xyz".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_xyz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xyz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopUvxform {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "uvxform"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait VopUvxformOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Transformed Texture Coordinates"
    fn out_xformuvw(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("xformuvw".to_string()),
        }
    }
}

impl VopUvxformOutputs for VopUvxform {}
impl VopUvxformOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopUvxform> {}
