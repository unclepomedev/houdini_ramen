#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWaveformFalloff {
    None = 0,
    Linear = 1,
    Quadratic = 2,
}

#[derive(Debug, Clone)]
pub struct SopWaveform {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopWaveform {
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

    /// Connects to input 0: "Geometry to Deform with Deep Ocean Wave"
    pub fn set_input_geometry_to_deform_with_deep_ocean_wave<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Deform with Deep Ocean Wave" and specifies the output index of the target node.
    pub fn set_input_geometry_to_deform_with_deep_ocean_wave_from<
        N: crate::core::types::HoudiniNode,
    >(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_h(mut self, val: f32) -> Self {
        self.params
            .insert("H".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_h_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "H".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_d(mut self, val: f32) -> Self {
        self.params
            .insert("d".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_d_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_width(mut self, val: f32) -> Self {
        self.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_width_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_g(mut self, val: f32) -> Self {
        self.params
            .insert("g".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_g_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "g".to_string(),
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
    pub fn with_constant_coefficient(mut self, val: f32) -> Self {
        self.params.insert(
            "constant_coefficient".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_constant_coefficient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constant_coefficient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_linear_coefficient(mut self, val: f32) -> Self {
        self.params.insert(
            "linear_coefficient".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_linear_coefficient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "linear_coefficient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quadratic_coefficient(mut self, val: f32) -> Self {
        self.params.insert(
            "quadratic_coefficient".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_quadratic_coefficient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quadratic_coefficient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
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

    // --- Int parameters ---
    pub fn with_falloff(mut self, val: SopWaveformFalloff) -> Self {
        self.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
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

    // --- Toggle parameters ---
    pub fn with_output_attrib(mut self, val: bool) -> Self {
        self.params.insert(
            "output_attrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_output_attrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "output_attrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopWaveform {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "waveform"
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
pub enum SopWavevelocityFalloff {
    None = 0,
    Linear = 1,
    Quadratic = 2,
}

#[derive(Debug, Clone)]
pub struct SopWavevelocity {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopWavevelocity {
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

    // --- Float parameters ---
    pub fn with_h(mut self, val: f32) -> Self {
        self.params
            .insert("H".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_h_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "H".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_d(mut self, val: f32) -> Self {
        self.params
            .insert("d".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_d_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_width(mut self, val: f32) -> Self {
        self.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_width_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_g(mut self, val: f32) -> Self {
        self.params
            .insert("g".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_g_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "g".to_string(),
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
    pub fn with_constant_coefficient(mut self, val: f32) -> Self {
        self.params.insert(
            "constant_coefficient".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_constant_coefficient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constant_coefficient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_linear_coefficient(mut self, val: f32) -> Self {
        self.params.insert(
            "linear_coefficient".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_linear_coefficient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "linear_coefficient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quadratic_coefficient(mut self, val: f32) -> Self {
        self.params.insert(
            "quadratic_coefficient".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_quadratic_coefficient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quadratic_coefficient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset(mut self, val: f32) -> Self {
        self.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float(val),
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

    // --- Float3 parameters ---
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

    // --- Int parameters ---
    pub fn with_falloff(mut self, val: SopWavevelocityFalloff) -> Self {
        self.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
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
}

impl crate::core::types::HoudiniNode for SopWavevelocity {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "wavevelocity"
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
pub enum SopWeightarraybiharmonicPrimtype {
    Triangles = 0,
    Tetrahedra = 1,
}

#[derive(Debug, Clone)]
pub struct SopWeightarraybiharmonic {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopWeightarraybiharmonic {
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

    /// Connects to input 0: "Guide Interpolation Geometry"
    pub fn set_input_guide_interpolation_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Guide Interpolation Geometry" and specifies the output index of the target node.
    pub fn set_input_guide_interpolation_geometry_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_difftol(mut self, val: f32) -> Self {
        self.params.insert(
            "difftol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_difftol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "difftol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_maxiter(mut self, val: i32) -> Self {
        self.params.insert(
            "maxiter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxiter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxiter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_primtype(mut self, val: SopWeightarraybiharmonicPrimtype) -> Self {
        self.params.insert(
            "primtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indexattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "indexattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indexattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indexattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weightattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "weightattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_weightattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weightattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_verbose(mut self, val: bool) -> Self {
        self.params.insert(
            "verbose".to_string(),
            crate::core::types::ParamValue::Toggle(val),
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
}

impl crate::core::types::HoudiniNode for SopWeightarraybiharmonic {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "weightarraybiharmonic"
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
pub struct SopWeightarrayinterpolate {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopWeightarrayinterpolate {
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

    /// Connects to input 0: "Skin"
    pub fn set_input_skin<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Guides"
    pub fn set_input_guides<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Guides" and specifies the output index of the target node.
    pub fn set_input_guides_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_primattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "primattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primuvattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "primuvattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primuvattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primuvattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indexattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "indexattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indexattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indexattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weightattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "weightattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_weightattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weightattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopWeightarrayinterpolate {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "weightarrayinterpolate"
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
pub enum SopWhitewaterpostprocessOutput {
    Particles = 0,
    FogVolume = 1,
    Mesh = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWhitewaterpostprocessFlattenplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWhitewaterpostprocessFlattenshape {
    Rectangle = 0,
    Circle = 1,
}

#[derive(Debug, Clone)]
pub struct SopWhitewaterpostprocess {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopWhitewaterpostprocess {
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

    /// Connects to input 0: "Whitewater Simulation"
    pub fn set_input_whitewater_simulation<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Whitewater Simulation" and specifies the output index of the target node.
    pub fn set_input_whitewater_simulation_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Container"
    pub fn set_input_container<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Container" and specifies the output index of the target node.
    pub fn set_input_container_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Collisions"
    pub fn set_input_collisions<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Collisions" and specifies the output index of the target node.
    pub fn set_input_collisions_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_voxelsize(mut self, val: f32) -> Self {
        self.params.insert(
            "voxelsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_voxelsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "voxelsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velmultiplier(mut self, val: f32) -> Self {
        self.params.insert(
            "velmultiplier".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velmultiplier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velmultiplier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_voxeloffset(mut self, val: f32) -> Self {
        self.params.insert(
            "voxeloffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_voxeloffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "voxeloffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adaptivity(mut self, val: f32) -> Self {
        self.params.insert(
            "adaptivity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_adaptivity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adaptivity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flattencolsuppress(mut self, val: f32) -> Self {
        self.params.insert(
            "flattencolsuppress".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_flattencolsuppress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flattencolsuppress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_waterlevel(mut self, val: f32) -> Self {
        self.params.insert(
            "waterlevel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_waterlevel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "waterlevel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flattendist(mut self, val: f32) -> Self {
        self.params.insert(
            "flattendist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_flattendist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flattendist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_density_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "density_range".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_density_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_agerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "density_agerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_density_agerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_agerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_nagerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "density_nagerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_density_nagerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_nagerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_depthrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "density_depthrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_density_depthrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_depthrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_distancerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "density_distancerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_density_distancerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_distancerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscale_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "pscale_range".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pscale_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscale_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscale_agerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "pscale_agerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pscale_agerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscale_agerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscale_nagerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "pscale_nagerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pscale_nagerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscale_nagerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscale_depthrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "pscale_depthrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pscale_depthrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscale_depthrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscale_distancerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "pscale_distancerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pscale_distancerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscale_distancerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_size(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Float3(val),
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

    // --- Int parameters ---
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_output(mut self, val: SopWhitewaterpostprocessOutput) -> Self {
        self.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flattenplane(mut self, val: SopWhitewaterpostprocessFlattenplane) -> Self {
        self.params.insert(
            "flattenplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_flattenplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flattenplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flattenshape(mut self, val: SopWhitewaterpostprocessFlattenshape) -> Self {
        self.params.insert(
            "flattenshape".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_flattenshape_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flattenshape".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_density_ageramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "density_ageramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_density_ageramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_ageramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_nageramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "density_nageramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_density_nageramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_nageramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_depthramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "density_depthramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_density_depthramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_depthramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_distanceramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "density_distanceramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_density_distanceramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_distanceramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscale_ageramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "pscale_ageramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_pscale_ageramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscale_ageramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscale_nageramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "pscale_nageramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_pscale_nageramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscale_nageramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscale_depthramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "pscale_depthramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_pscale_depthramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscale_depthramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscale_distanceramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "pscale_distanceramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_pscale_distanceramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscale_distanceramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_displayassphere(mut self, val: bool) -> Self {
        self.params.insert(
            "displayassphere".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayassphere_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayassphere".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setalphafromdensity(mut self, val: bool) -> Self {
        self.params.insert(
            "setalphafromdensity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setalphafromdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setalphafromdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addgradient(mut self, val: bool) -> Self {
        self.params.insert(
            "addgradient".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addgradient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addgradient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vdbhalf(mut self, val: bool) -> Self {
        self.params.insert(
            "vdbhalf".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vdbhalf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vdbhalf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_set(mut self, val: bool) -> Self {
        self.params.insert(
            "density_set".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_set_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_set".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_byage(mut self, val: bool) -> Self {
        self.params.insert(
            "density_byage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_byage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_byage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_bynage(mut self, val: bool) -> Self {
        self.params.insert(
            "density_bynage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_bynage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_bynage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_bydepth(mut self, val: bool) -> Self {
        self.params.insert(
            "density_bydepth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_bydepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_bydepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_bydomain(mut self, val: bool) -> Self {
        self.params.insert(
            "density_bydomain".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_bydomain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_bydomain".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscale_set(mut self, val: bool) -> Self {
        self.params.insert(
            "pscale_set".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pscale_set_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscale_set".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscale_byage(mut self, val: bool) -> Self {
        self.params.insert(
            "pscale_byage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pscale_byage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscale_byage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscale_bynage(mut self, val: bool) -> Self {
        self.params.insert(
            "pscale_bynage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pscale_bynage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscale_bynage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscale_bydepth(mut self, val: bool) -> Self {
        self.params.insert(
            "pscale_bydepth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pscale_bydepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscale_bydepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscale_bydomain(mut self, val: bool) -> Self {
        self.params.insert(
            "pscale_bydomain".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pscale_bydomain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pscale_bydomain".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipcontainer(mut self, val: bool) -> Self {
        self.params.insert(
            "clipcontainer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clipcontainer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clipcontainer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flattengeo(mut self, val: bool) -> Self {
        self.params.insert(
            "flattengeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flattengeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flattengeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flattenattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "flattenattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flattenattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flattenattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flattendocolsuppress(mut self, val: bool) -> Self {
        self.params.insert(
            "flattendocolsuppress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flattendocolsuppress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flattendocolsuppress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopWhitewaterpostprocess {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "whitewaterpostprocess"
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
pub enum SopWhitewatersolverVisMode {
    None = 0,
    Depth = 1,
    Speed = 2,
    State = 3,
    RelativeDensity = 4,
}

#[derive(Debug, Clone)]
pub struct SopWhitewatersolver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopWhitewatersolver {
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

    /// Connects to input 0: "Emission and Fluid Fields"
    pub fn set_input_emission_and_fluid_fields<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Emission and Fluid Fields" and specifies the output index of the target node.
    pub fn set_input_emission_and_fluid_fields_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Container"
    pub fn set_input_container<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Container" and specifies the output index of the target node.
    pub fn set_input_container_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Collisions"
    pub fn set_input_collisions<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Collisions" and specifies the output index of the target node.
    pub fn set_input_collisions_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_resimulate(mut self) -> Self {
        self.params.insert(
            "resimulate".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_checkpoint_openbasedir(mut self) -> Self {
        self.params.insert(
            "checkpoint_openbasedir".to_string(),
            crate::core::types::ParamValue::Button,
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
    pub fn with_voxelsize(mut self, val: f32) -> Self {
        self.params.insert(
            "voxelsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_voxelsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "voxelsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goaldepth(mut self, val: f32) -> Self {
        self.params.insert(
            "goaldepth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_goaldepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goaldepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthrange(mut self, val: f32) -> Self {
        self.params.insert(
            "depthrange".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_depthrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounceforward(mut self, val: f32) -> Self {
        self.params.insert(
            "bounceforward".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounceforward_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounceforward".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_friction(mut self, val: f32) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_friction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dynamicfriction(mut self, val: f32) -> Self {
        self.params.insert(
            "dynamicfriction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dynamicfriction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dynamicfriction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emissionamount(mut self, val: f32) -> Self {
        self.params.insert(
            "emissionamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_emissionamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emissionamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_veloffset(mut self, val: f32) -> Self {
        self.params.insert(
            "veloffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_veloffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "veloffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocitymultiplier(mut self, val: f32) -> Self {
        self.params.insert(
            "velocitymultiplier".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velocitymultiplier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocitymultiplier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxspeed(mut self, val: f32) -> Self {
        self.params.insert(
            "maxspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lifespan(mut self, val: f32) -> Self {
        self.params.insert(
            "lifespan".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lifespan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lifespan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bubblesagingrate(mut self, val: f32) -> Self {
        self.params.insert(
            "bubblesagingrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bubblesagingrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bubblesagingrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_foamagingrate(mut self, val: f32) -> Self {
        self.params.insert(
            "foamagingrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_foamagingrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foamagingrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sprayagingrate(mut self, val: f32) -> Self {
        self.params.insert(
            "sprayagingrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sprayagingrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sprayagingrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amp(mut self, val: f32) -> Self {
        self.params.insert(
            "amp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "rough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rough".to_string(),
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
    pub fn with_resistance(mut self, val: f32) -> Self {
        self.params.insert(
            "resistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_resistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_multiplier(mut self, val: f32) -> Self {
        self.params.insert(
            "multiplier".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_multiplier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "multiplier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windspeed(mut self, val: f32) -> Self {
        self.params.insert(
            "windspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_windspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "windspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_airresist(mut self, val: f32) -> Self {
        self.params.insert(
            "airresist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_airresist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "airresist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wind_amp(mut self, val: f32) -> Self {
        self.params.insert(
            "wind_amp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wind_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wind_amp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wind_swirlsize(mut self, val: f32) -> Self {
        self.params.insert(
            "wind_swirlsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wind_swirlsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wind_swirlsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wind_shadowmaxdistance(mut self, val: f32) -> Self {
        self.params.insert(
            "wind_shadowmaxdistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wind_shadowmaxdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wind_shadowmaxdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "defstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_defstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pbfinclusion(mut self, val: f32) -> Self {
        self.params.insert(
            "pbfinclusion".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pbfinclusion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pbfinclusion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxacceleration(mut self, val: f32) -> Self {
        self.params.insert(
            "maxacceleration".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxacceleration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxacceleration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tensileq(mut self, val: f32) -> Self {
        self.params.insert(
            "tensileq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tensileq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tensileq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tensilek(mut self, val: f32) -> Self {
        self.params.insert(
            "tensilek".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tensilek_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tensilek".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosityc(mut self, val: f32) -> Self {
        self.params.insert(
            "viscosityc".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosityc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosityc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_erosionrange(mut self, val: f32) -> Self {
        self.params.insert(
            "erosionrange".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_erosionrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "erosionrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_erosionstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "erosionstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_erosionstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "erosionstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preservationstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "preservationstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_preservationstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preservationstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reppulserange(mut self, val: f32) -> Self {
        self.params.insert(
            "reppulserange".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reppulserange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reppulserange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repdensitythreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "repdensitythreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_repdensitythreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repdensitythreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repellantseed(mut self, val: f32) -> Self {
        self.params.insert(
            "repellantseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_repellantseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repellantseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthcontrolrange(mut self, val: f32) -> Self {
        self.params.insert(
            "depthcontrolrange".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_depthcontrolrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthcontrolrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velangle(mut self, val: f32) -> Self {
        self.params.insert(
            "velangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_max_speed(mut self, val: f32) -> Self {
        self.params.insert(
            "max_speed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_max_speed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "max_speed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timescale(mut self, val: f32) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_projectionrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "projectionrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_projectionrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projectionrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wind_speedrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "wind_speedrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_wind_speedrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wind_speedrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wind_relativedensityrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "wind_relativedensityrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_wind_relativedensityrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wind_relativedensityrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pbfdepthrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "pbfdepthrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pbfdepthrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pbfdepthrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kernelradiusrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "kernelradiusrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_kernelradiusrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernelradiusrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repfeatsizerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "repfeatsizerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_repfeatsizerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repfeatsizerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repmagrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "repmagrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_repmagrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repmagrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repnoiserange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "repnoiserange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_repnoiserange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repnoiserange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repliferange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "repliferange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_repliferange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repliferange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repspeedrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "repspeedrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_repspeedrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repspeedrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repaccelrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "repaccelrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_repaccelrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repaccelrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depth_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "depth_range".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_depth_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depth_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
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
    pub fn with_gravity(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gravity".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gravity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gravity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_buoyancy(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "buoyancy".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_buoyancy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "buoyancy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wind(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "wind".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_wind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wind".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bubble_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "bubble_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bubble_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bubble_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_foam_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "foam_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_foam_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foam_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spray_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spray_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spray_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spray_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_startframe(mut self, val: i32) -> Self {
        self.params.insert(
            "startframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_startframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "startframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_substep(mut self, val: i32) -> Self {
        self.params.insert(
            "substep".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "substep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachemaxsize(mut self, val: i32) -> Self {
        self.params.insert(
            "cachemaxsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachemaxsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachemaxsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_turb(mut self, val: i32) -> Self {
        self.params
            .insert("turb".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_turb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "turb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_desiredneighbors(mut self, val: i32) -> Self {
        self.params.insert(
            "desiredneighbors".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_desiredneighbors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "desiredneighbors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxneighbors(mut self, val: i32) -> Self {
        self.params.insert(
            "maxneighbors".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxneighbors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxneighbors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraintiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "constraintiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_constraintiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraintiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repseedframes(mut self, val: i32) -> Self {
        self.params.insert(
            "repseedframes".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_repseedframes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repseedframes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdfhalfwidth(mut self, val: i32) -> Self {
        self.params.insert(
            "sdfhalfwidth".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sdfhalfwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdfhalfwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpoint_version(mut self, val: i32) -> Self {
        self.params.insert(
            "checkpoint_version".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_checkpoint_version_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpoint_version".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpoint_explicitcachensteps(mut self, val: i32) -> Self {
        self.params.insert(
            "checkpoint_explicitcachensteps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_checkpoint_explicitcachensteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpoint_explicitcachensteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpoint_explicitcachespacing(mut self, val: i32) -> Self {
        self.params.insert(
            "checkpoint_explicitcachespacing".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_checkpoint_explicitcachespacing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpoint_explicitcachespacing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_vis_mode(mut self, val: SopWhitewatersolverVisMode) -> Self {
        self.params.insert(
            "vis_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vis_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_buoyancycurve(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "buoyancycurve".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_buoyancycurve_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "buoyancycurve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_advectioncurve(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "advectioncurve".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_advectioncurve_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "advectioncurve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_multipliercurve(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "multipliercurve".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_multipliercurve_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "multipliercurve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wind_speedramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "wind_speedramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_wind_speedramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wind_speedramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wind_relativedensityramp(
        mut self,
        val: Vec<crate::core::types::RampPoint>,
    ) -> Self {
        self.params.insert(
            "wind_relativedensityramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_wind_relativedensityramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wind_relativedensityramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stiffnesscurve(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "stiffnesscurve".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_stiffnesscurve_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stiffnesscurve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raddistributionramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "raddistributionramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_raddistributionramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raddistributionramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strengthramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "strengthramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_strengthramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strengthramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_accelerationramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "accelerationramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_accelerationramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accelerationramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repulsioncurve(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "repulsioncurve".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_repulsioncurve_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repulsioncurve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adhesionstiffnesscurve(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "adhesionstiffnesscurve".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_adhesionstiffnesscurve_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adhesionstiffnesscurve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depth_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "depth_ramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_depth_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depth_ramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_speed_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "speed_ramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_speed_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speed_ramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rel_dens_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "rel_dens_ramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_rel_dens_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rel_dens_ramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_particlekeepattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "particlekeepattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_particlekeepattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particlekeepattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_type(mut self, val: &str) -> Self {
        self.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_checkpoint_basename(mut self, val: &str) -> Self {
        self.params.insert(
            "checkpoint_basename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_checkpoint_basename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpoint_basename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpoint_basedir(mut self, val: &str) -> Self {
        self.params.insert(
            "checkpoint_basedir".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_checkpoint_basedir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpoint_basedir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_doparticlekeepattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "doparticlekeepattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doparticlekeepattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doparticlekeepattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addstatevars(mut self, val: bool) -> Self {
        self.params.insert(
            "addstatevars".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addstatevars_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addstatevars".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adddensityvar(mut self, val: bool) -> Self {
        self.params.insert(
            "adddensityvar".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adddensityvar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adddensityvar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cacheenabled(mut self, val: bool) -> Self {
        self.params.insert(
            "cacheenabled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cacheenabled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cacheenabled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limitemission(mut self, val: bool) -> Self {
        self.params.insert(
            "limitemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limitemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_projectemission(mut self, val: bool) -> Self {
        self.params.insert(
            "projectemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_projectemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projectemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_enable_wind(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_wind".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_wind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_wind".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wind_enableshadow(mut self, val: bool) -> Self {
        self.params.insert(
            "wind_enableshadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_wind_enableshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wind_enableshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wind_enablecollidershadow(mut self, val: bool) -> Self {
        self.params.insert(
            "wind_enablecollidershadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_wind_enablecollidershadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wind_enablecollidershadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wind_enablewatershadow(mut self, val: bool) -> Self {
        self.params.insert(
            "wind_enablewatershadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_wind_enablewatershadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wind_enablewatershadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledensitycontrol(mut self, val: bool) -> Self {
        self.params.insert(
            "enabledensitycontrol".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledensitycontrol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledensitycontrol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablexpbdstiffness(mut self, val: bool) -> Self {
        self.params.insert(
            "enablexpbdstiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablexpbdstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablexpbdstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adaptivekernelradius(mut self, val: bool) -> Self {
        self.params.insert(
            "adaptivekernelradius".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adaptivekernelradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adaptivekernelradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableviscosity(mut self, val: bool) -> Self {
        self.params.insert(
            "enableviscosity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableviscosity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableviscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableerosion(mut self, val: bool) -> Self {
        self.params.insert(
            "enableerosion".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableerosion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableerosion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablerepellants(mut self, val: bool) -> Self {
        self.params.insert(
            "enablerepellants".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablerepellants_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablerepellants".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mortalrepellants(mut self, val: bool) -> Self {
        self.params.insert(
            "mortalrepellants".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mortalrepellants_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mortalrepellants".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repseedbydensity(mut self, val: bool) -> Self {
        self.params.insert(
            "repseedbydensity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_repseedbydensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repseedbydensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raddistribution(mut self, val: bool) -> Self {
        self.params.insert(
            "raddistribution".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_raddistribution_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raddistribution".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attenuatebyspeed(mut self, val: bool) -> Self {
        self.params.insert(
            "attenuatebyspeed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_attenuatebyspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attenuatebyspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attenuatebyaccel(mut self, val: bool) -> Self {
        self.params.insert(
            "attenuatebyaccel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_attenuatebyaccel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attenuatebyaccel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledepthcontrol(mut self, val: bool) -> Self {
        self.params.insert(
            "enabledepthcontrol".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledepthcontrol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledepthcontrol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthcontrolgrav(mut self, val: bool) -> Self {
        self.params.insert(
            "depthcontrolgrav".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_depthcontrolgrav_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthcontrolgrav".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showcollision(mut self, val: bool) -> Self {
        self.params.insert(
            "showcollision".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showcollision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showcollision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdfactivate(mut self, val: bool) -> Self {
        self.params.insert(
            "sdfactivate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sdfactivate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdfactivate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useopencl(mut self, val: bool) -> Self {
        self.params.insert(
            "useopencl".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useopencl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useopencl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savecheckpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "savecheckpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savecheckpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savecheckpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpoint_enableversion(mut self, val: bool) -> Self {
        self.params.insert(
            "checkpoint_enableversion".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_checkpoint_enableversion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpoint_enableversion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopWhitewatersolver {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "whitewatersolver"
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
        Some("dopnet1/FORCES")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait SopWhitewatersolverInnerExt {
    fn extra_sources(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn particle_forces(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn post_solve(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> SopWhitewatersolverInnerExt for crate::core::graph::InnerGraph<'a> {
    fn extra_sources(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("dopnet1/FORCES/EXTRA_SOURCES")
    }
    fn particle_forces(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("dopnet1/FORCES/PARTICLE_FORCES")
    }
    fn post_solve(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("dopnet1/FORCES/POST_SOLVE")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWhitewatersourceMergemethod {
    Add = 0,
    Maximum = 1,
    Override = 2,
}

#[derive(Debug, Clone)]
pub struct SopWhitewatersource {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopWhitewatersource {
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

    /// Connects to input 0: "Liquid Simulation"
    pub fn set_input_liquid_simulation<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Liquid Simulation" and specifies the output index of the target node.
    pub fn set_input_liquid_simulation_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Container"
    pub fn set_input_container<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Container" and specifies the output index of the target node.
    pub fn set_input_container_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Collisions"
    pub fn set_input_collisions<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Collisions" and specifies the output index of the target node.
    pub fn set_input_collisions_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Extra Sources"
    pub fn set_input_extra_sources<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Extra Sources" and specifies the output index of the target node.
    pub fn set_input_extra_sources_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_computecurvaturerange(mut self) -> Self {
        self.params.insert(
            "computecurvaturerange".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_computeaccelerationrange(mut self) -> Self {
        self.params.insert(
            "computeaccelerationrange".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_computevorticityrange(mut self) -> Self {
        self.params.insert(
            "computevorticityrange".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_computesquishrange(mut self) -> Self {
        self.params.insert(
            "computesquishrange".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_computestretchrange(mut self) -> Self {
        self.params.insert(
            "computestretchrange".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_computescalerange(mut self) -> Self {
        self.params.insert(
            "computescalerange".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_voxelsize(mut self, val: f32) -> Self {
        self.params.insert(
            "voxelsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_voxelsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "voxelsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxvelangle(mut self, val: f32) -> Self {
        self.params.insert(
            "maxvelangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxvelangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxvelangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressurescale(mut self, val: f32) -> Self {
        self.params.insert(
            "pressurescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pressurescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressurescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splashscale(mut self, val: f32) -> Self {
        self.params.insert(
            "splashscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_splashscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splashscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_squishscale(mut self, val: f32) -> Self {
        self.params.insert(
            "squishscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_squishscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "squishscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchscale(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacescalescale(mut self, val: f32) -> Self {
        self.params.insert(
            "surfacescalescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacescalescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfacescalescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particlescale(mut self, val: f32) -> Self {
        self.params.insert(
            "particlescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particlescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particlescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wwscale(mut self, val: f32) -> Self {
        self.params.insert(
            "wwscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wwscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wwscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emissionamount(mut self, val: f32) -> Self {
        self.params.insert(
            "emissionamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_emissionamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emissionamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_depthrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "depthrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_depthrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_speedrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "speedrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_speedrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speedrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curvaturerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "curvaturerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_curvaturerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvaturerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_accelerationrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "accelerationrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_accelerationrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accelerationrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vorticityrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "vorticityrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_vorticityrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vorticityrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_squishrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "squishrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_squishrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "squishrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "stretchrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_stretchrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacescalerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "surfacescalerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_surfacescalerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfacescalerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_curvaturecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "curvaturecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_curvaturecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvaturecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_accelerationcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "accelerationcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_accelerationcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accelerationcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vorticitycolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vorticitycolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vorticitycolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vorticitycolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressurecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pressurecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pressurecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressurecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deformationcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "deformationcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_deformationcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deformationcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splash_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "splash_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_splash_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splash_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extracolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "extracolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_extracolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extracolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_startframe(mut self, val: i32) -> Self {
        self.params.insert(
            "startframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_startframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "startframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxhalfwidth(mut self, val: i32) -> Self {
        self.params.insert(
            "maxhalfwidth".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxhalfwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxhalfwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dilatevoxels(mut self, val: i32) -> Self {
        self.params.insert(
            "dilatevoxels".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_dilatevoxels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dilatevoxels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_erodevoxels(mut self, val: i32) -> Self {
        self.params.insert(
            "erodevoxels".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_erodevoxels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "erodevoxels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emergencylimit(mut self, val: i32) -> Self {
        self.params.insert(
            "emergencylimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_emergencylimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emergencylimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_mergemethod(mut self, val: SopWhitewatersourceMergemethod) -> Self {
        self.params.insert(
            "mergemethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mergemethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mergemethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_speedramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "speedramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_speedramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speedramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curvatureramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "curvatureramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_curvatureramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvatureramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_accelerationramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "accelerationramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_accelerationramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accelerationramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vorticityramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "vorticityramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_vorticityramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vorticityramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressureramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "pressureramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_pressureramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressureramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relativedensityramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "relativedensityramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_relativedensityramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relativedensityramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_squishramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "squishramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_squishramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "squishramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "stretchramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_stretchramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacescaleramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "surfacescaleramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_surfacescaleramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfacescaleramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emissionramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "emissionramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_emissionramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emissionramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emitattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "emitattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_emitattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emitattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_operation(mut self, val: &str) -> Self {
        self.params.insert(
            "operation".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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

    // --- Toggle parameters ---
    pub fn with_usevoxelsize(mut self, val: bool) -> Self {
        self.params.insert(
            "usevoxelsize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usevoxelsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usevoxelsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limitbydepth(mut self, val: bool) -> Self {
        self.params.insert(
            "limitbydepth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitbydepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limitbydepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableactivate(mut self, val: bool) -> Self {
        self.params.insert(
            "enableactivate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableactivate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableactivate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remapspeed(mut self, val: bool) -> Self {
        self.params.insert(
            "remapspeed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remapspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remapspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecurvature(mut self, val: bool) -> Self {
        self.params.insert(
            "enablecurvature".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecurvature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablecurvature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remapcurvature(mut self, val: bool) -> Self {
        self.params.insert(
            "remapcurvature".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remapcurvature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remapcurvature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableacceleration(mut self, val: bool) -> Self {
        self.params.insert(
            "enableacceleration".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableacceleration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableacceleration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remapacceleration(mut self, val: bool) -> Self {
        self.params.insert(
            "remapacceleration".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remapacceleration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remapacceleration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablevorticity(mut self, val: bool) -> Self {
        self.params.insert(
            "enablevorticity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablevorticity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablevorticity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remapvorticity(mut self, val: bool) -> Self {
        self.params.insert(
            "remapvorticity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remapvorticity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remapvorticity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablepressure(mut self, val: bool) -> Self {
        self.params.insert(
            "enablepressure".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablepressure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablepressure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remappressure(mut self, val: bool) -> Self {
        self.params.insert(
            "remappressure".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remappressure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remappressure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesplash(mut self, val: bool) -> Self {
        self.params.insert(
            "enablesplash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesplash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablesplash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doreplicate(mut self, val: bool) -> Self {
        self.params.insert(
            "doreplicate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doreplicate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doreplicate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relativedensityremap(mut self, val: bool) -> Self {
        self.params.insert(
            "relativedensityremap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_relativedensityremap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relativedensityremap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledeformation(mut self, val: bool) -> Self {
        self.params.insert(
            "enabledeformation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledeformation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledeformation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remapsquish(mut self, val: bool) -> Self {
        self.params.insert(
            "remapsquish".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remapsquish_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remapsquish".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remapstretch(mut self, val: bool) -> Self {
        self.params.insert(
            "remapstretch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remapstretch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remapstretch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remapsurfacescale(mut self, val: bool) -> Self {
        self.params.insert(
            "remapsurfacescale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remapsurfacescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remapsurfacescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remapemission(mut self, val: bool) -> Self {
        self.params.insert(
            "remapemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remapemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remapemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputvolumes(mut self, val: bool) -> Self {
        self.params.insert(
            "outputvolumes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputvolumes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputvolumes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modifyvolumes(mut self, val: bool) -> Self {
        self.params.insert(
            "modifyvolumes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_modifyvolumes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modifyvolumes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodilate(mut self, val: bool) -> Self {
        self.params.insert(
            "dodilate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodilate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dodilate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dosmooth(mut self, val: bool) -> Self {
        self.params.insert(
            "dosmooth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosmooth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dosmooth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doerode(mut self, val: bool) -> Self {
        self.params.insert(
            "doerode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doerode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doerode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visemitvol(mut self, val: bool) -> Self {
        self.params.insert(
            "visemitvol".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visemitvol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visemitvol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputvelattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "outputvelattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputvelattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputvelattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputemitattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "outputemitattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputemitattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputemitattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopWhitewatersource {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "whitewatersource"
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
pub enum SopWindingnumberType {
    /// 3D
    N3d = 0,
    /// 2D in XY Plane
    N2dInXyPlane = 1,
    /// 2D in YZ Plane
    N2dInYzPlane = 2,
    /// 2D in ZX Plane
    N2dInZxPlane = 3,
}

#[derive(Debug, Clone)]
pub struct SopWindingnumber {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopWindingnumber {
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

    /// Connects to input 0: "Query Points"
    pub fn set_input_query_points<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Query Points" and specifies the output index of the target node.
    pub fn set_input_query_points_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Occlusion Mesh"
    pub fn set_input_occlusion_mesh<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Occlusion Mesh" and specifies the output index of the target node.
    pub fn set_input_occlusion_mesh_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_accuracyscale(mut self, val: f32) -> Self {
        self.params.insert(
            "accuracyscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_accuracyscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accuracyscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type(mut self, val: SopWindingnumberType) -> Self {
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

    // --- String parameters ---
    pub fn with_querypoints(mut self, val: &str) -> Self {
        self.params.insert(
            "querypoints".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_querypoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "querypoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_meshprims(mut self, val: &str) -> Self {
        self.params.insert(
            "meshprims".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_meshprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "meshprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attrib(mut self, val: &str) -> Self {
        self.params.insert(
            "attrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_assolidangle(mut self, val: bool) -> Self {
        self.params.insert(
            "assolidangle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_assolidangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "assolidangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_negate(mut self, val: bool) -> Self {
        self.params.insert(
            "negate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_negate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "negate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fullaccuracy(mut self, val: bool) -> Self {
        self.params.insert(
            "fullaccuracy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fullaccuracy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fullaccuracy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopWindingnumber {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "windingnumber"
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
pub enum SopWireUsescaleattrib {
    Constant = 0,
    Attribute = 1,
}

#[derive(Debug, Clone)]
pub struct SopWire {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopWire {
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

    /// Connects to input 0: "Source data"
    pub fn set_input_source_data<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Source data" and specifies the output index of the target node.
    pub fn set_input_source_data_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_usescaleattrib(mut self, val: SopWireUsescaleattrib) -> Self {
        self.params.insert(
            "usescaleattrib".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_usescaleattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usescaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scaleattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "scaleattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scaleattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_corners(mut self, val: bool) -> Self {
        self.params.insert(
            "corners".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_corners_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "corners".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_caps(mut self, val: bool) -> Self {
        self.params.insert(
            "caps".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "caps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remove(mut self, val: bool) -> Self {
        self.params.insert(
            "remove".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remove_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remove".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopWire {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "wire"
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
pub struct SopWireblend {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl SopWireblend {
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

    // --- Float parameters ---
    pub fn with_blend_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("blend{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("blend{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_diff(mut self, val: bool) -> Self {
        self.params.insert(
            "diff".to_string(),
            crate::core::types::ParamValue::Toggle(val),
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
}

impl crate::core::types::HoudiniNode for SopWireblend {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "wireblend"
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
pub enum SopWirecaptureVistype {
    SinglePrimitive = 0,
    MultiplePrimitives = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWirecaptureVismode {
    /// Infra-Red
    InfraMinusRed = 0,
    WhiteToRed = 1,
    Grayscale = 2,
    Blackbody = 3,
}

#[derive(Debug, Clone)]
pub struct SopWirecapture {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopWirecapture {
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

    /// Connects to input 0: "Geometry to capture"
    pub fn set_input_geometry_to_capture<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to capture" and specifies the output index of the target node.
    pub fn set_input_geometry_to_capture_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Wires to capture with"
    pub fn set_input_wires_to_capture_with<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Wires to capture with" and specifies the output index of the target node.
    pub fn set_input_wires_to_capture_with_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_init(mut self) -> Self {
        self.params
            .insert("init".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_initgroup(mut self) -> Self {
        self.params.insert(
            "initgroup".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_blendfactor(mut self, val: f32) -> Self {
        self.params.insert(
            "blendfactor".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blendfactor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blendfactor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffstarti_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("falloffstart{}i", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_falloffstarti_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("falloffstart{}i", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_wrange_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(
            format!("w{}range", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_wrange_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("w{}range", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uradius_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(
            format!("u{}radius", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_uradius_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("u{}radius", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_urange_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(
            format!("u{}range", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_urange_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("u{}range", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_zeroweightcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "zeroweightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_zeroweightcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zeroweightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_vistype(mut self, val: SopWirecaptureVistype) -> Self {
        self.params.insert(
            "vistype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vistype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vistype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vismode(mut self, val: SopWirecaptureVismode) -> Self {
        self.params.insert(
            "vismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kernel(mut self, val: &str) -> Self {
        self.params.insert(
            "kernel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kernel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prim_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("prim{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prim_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("prim{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subgroup_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("subgroup{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subgroup_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("subgroup{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_polyline(mut self, val: bool) -> Self {
        self.params.insert(
            "polyline".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_polyline_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "polyline".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_destroyweights(mut self, val: bool) -> Self {
        self.params.insert(
            "destroyweights".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_destroyweights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destroyweights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visualize(mut self, val: bool) -> Self {
        self.params.insert(
            "visualize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visualize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visualize".to_string(),
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
}

impl crate::core::types::HoudiniNode for SopWirecapture {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "wirecapture"
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
pub enum SopWiredeformBlend {
    MaxDisplace = 0,
    BiasedDisplace = 1,
    Weighted = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWiredeformXformframe {
    NoTransform = 0,
    AlignTangentOnly = 1,
    AlignTangentAndAttribNormal = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWiredeformVistype {
    SinglePrimitive = 0,
    MultiplePrimitives = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWiredeformVismode {
    /// Infra-Red
    InfraMinusRed = 0,
    WhiteToRed = 1,
    Grayscale = 2,
    Blackbody = 3,
}

#[derive(Debug, Clone)]
pub struct SopWiredeform {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopWiredeform {
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

    /// Connects to input 0: "Geometry to deform"
    pub fn set_input_geometry_to_deform<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to deform" and specifies the output index of the target node.
    pub fn set_input_geometry_to_deform_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Rest wires"
    pub fn set_input_rest_wires<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Rest wires" and specifies the output index of the target node.
    pub fn set_input_rest_wires_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Deforming wires"
    pub fn set_input_deforming_wires<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Deforming wires" and specifies the output index of the target node.
    pub fn set_input_deforming_wires_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_init(mut self) -> Self {
        self.params
            .insert("init".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_initgroup(mut self) -> Self {
        self.params.insert(
            "initgroup".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_tension(mut self, val: f32) -> Self {
        self.params.insert(
            "tension".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tension_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tension".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("scale{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("scale{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_udeform_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(
            format!("udeform{}", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_udeform_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("udeform{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_zeroweightcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "zeroweightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_zeroweightcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zeroweightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_blend(mut self, val: SopWiredeformBlend) -> Self {
        self.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xformframe(mut self, val: SopWiredeformXformframe) -> Self {
        self.params.insert(
            "xformframe".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xformframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vistype(mut self, val: SopWiredeformVistype) -> Self {
        self.params.insert(
            "vistype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vistype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vistype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vismode(mut self, val: SopWiredeformVismode) -> Self {
        self.params.insert(
            "vismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "restgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prim_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("prim{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prim_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("prim{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_polyline(mut self, val: bool) -> Self {
        self.params.insert(
            "polyline".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_polyline_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "polyline".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updatenmls(mut self, val: bool) -> Self {
        self.params.insert(
            "updatenmls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_updatenmls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updatenmls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fastrecache(mut self, val: bool) -> Self {
        self.params.insert(
            "fastrecache".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fastrecache_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fastrecache".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visualize(mut self, val: bool) -> Self {
        self.params.insert(
            "visualize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visualize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visualize".to_string(),
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
    pub fn with_clamp_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("clamp{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clamp_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("clamp{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopWiredeform {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "wiredeform"
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
pub struct SopWiretransfershape {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopWiretransfershape {
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

    /// Connects to input 0: "Curves to transfer to"
    pub fn set_input_curves_to_transfer_to<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Curves to transfer to" and specifies the output index of the target node.
    pub fn set_input_curves_to_transfer_to_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Curves to transfer from"
    pub fn set_input_curves_to_transfer_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Curves to transfer from" and specifies the output index of the target node.
    pub fn set_input_curves_to_transfer_from_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_srcgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "srcgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_srcgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dstgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "dstgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dstgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dstgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kernel(mut self, val: &str) -> Self {
        self.params.insert(
            "kernel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kernel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kernel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopWiretransfershape {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "wiretransfershape"
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
pub enum SopWrinkledeformerGrouptype {
    Primitives = 0,
    Points = 1,
    Edges = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWrinkledeformerTriangulation {
    None = 0,
    Regular = 1,
    Alternating = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWrinkledeformerDomass {
    Unchanged = 0,
    SetUniform = 1,
    CalculateUniform = 2,
    CalculateVarying = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWrinkledeformerDensityscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWrinkledeformerDothickness {
    Unchanged = 0,
    SetUniform = 1,
    CalculateUniform = 2,
    CalculateVarying = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWrinkledeformerThicknessscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWrinkledeformerConstrainttopology {
    Cloth = 0,
    SurfaceStruts = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWrinkledeformerRestlengthscalescalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWrinkledeformerCompressstiffnessscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWrinkledeformerStretchstiffnessscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWrinkledeformerAttribsource {
    DeformedGeometry = 0,
    RestGeometry = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWrinkledeformerInsetscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWrinkledeformerUseground {
    None = 0,
    GroundPlane = 1,
    HeightField = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWrinkledeformerDosdfoffset {
    SetUniform = 0,
    UseThickness = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWrinkledeformerWrinklescalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopWrinkledeformerVisMode {
    None = 0,
    Stress = 1,
    Distance = 2,
    Ratio = 3,
}

#[derive(Debug, Clone)]
pub struct SopWrinkledeformer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl SopWrinkledeformer {
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

    /// Connects to input 0: "Geometry to Deform"
    pub fn set_input_geometry_to_deform<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Deform" and specifies the output index of the target node.
    pub fn set_input_geometry_to_deform_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Rest Geometry"
    pub fn set_input_rest_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Rest Geometry" and specifies the output index of the target node.
    pub fn set_input_rest_geometry_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Collision Geometry"
    pub fn set_input_collision_geometry<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Collision Geometry" and specifies the output index of the target node.
    pub fn set_input_collision_geometry_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_mass(mut self, val: f32) -> Self {
        self.params.insert(
            "mass".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_thicknessscale(mut self, val: f32) -> Self {
        self.params.insert(
            "thicknessscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_thicknessscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thicknessscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restlengthscale(mut self, val: f32) -> Self {
        self.params.insert(
            "restlengthscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restlengthscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restlengthscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compressstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "compressstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_compressstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compressstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normalinset(mut self, val: f32) -> Self {
        self.params.insert(
            "normalinset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normalinset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalinset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdfoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "sdfoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sdfoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdfoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalewrinkle(mut self, val: f32) -> Self {
        self.params.insert(
            "scalewrinkle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scalewrinkle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalewrinkle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxangle(mut self, val: f32) -> Self {
        self.params.insert(
            "maxangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_maxstretchstress(mut self, val: f32) -> Self {
        self.params.insert(
            "vis_maxstretchstress".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vis_maxstretchstress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_maxstretchstress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_maxstretchdistance(mut self, val: f32) -> Self {
        self.params.insert(
            "vis_maxstretchdistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vis_maxstretchdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_maxstretchdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_maxstretchratio(mut self, val: f32) -> Self {
        self.params.insert(
            "vis_maxstretchratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vis_maxstretchratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_maxstretchratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_groundpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "groundpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_groundpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "groundpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_groundup(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "groundup".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_groundup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "groundup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_constraintiter(mut self, val: i32) -> Self {
        self.params.insert(
            "constraintiter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_constraintiter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraintiter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_neighbordist(mut self, val: i32) -> Self {
        self.params.insert(
            "neighbordist".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_neighbordist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "neighbordist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deltaiter(mut self, val: i32) -> Self {
        self.params.insert(
            "deltaiter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deltaiter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deltaiter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopWrinkledeformerGrouptype) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_triangulation(mut self, val: SopWrinkledeformerTriangulation) -> Self {
        self.params.insert(
            "triangulation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_triangulation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triangulation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domass(mut self, val: SopWrinkledeformerDomass) -> Self {
        self.params.insert(
            "domass".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_domass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_densityscalemode(mut self, val: SopWrinkledeformerDensityscalemode) -> Self {
        self.params.insert(
            "densityscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_densityscalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "densityscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dothickness(mut self, val: SopWrinkledeformerDothickness) -> Self {
        self.params.insert(
            "dothickness".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dothickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dothickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thicknessscalemode(mut self, val: SopWrinkledeformerThicknessscalemode) -> Self {
        self.params.insert(
            "thicknessscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_thicknessscalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thicknessscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainttopology(mut self, val: SopWrinkledeformerConstrainttopology) -> Self {
        self.params.insert(
            "constrainttopology".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_constrainttopology_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainttopology".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restlengthscalescalemode(
        mut self,
        val: SopWrinkledeformerRestlengthscalescalemode,
    ) -> Self {
        self.params.insert(
            "restlengthscalescalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_restlengthscalescalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restlengthscalescalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compressstiffnessscalemode(
        mut self,
        val: SopWrinkledeformerCompressstiffnessscalemode,
    ) -> Self {
        self.params.insert(
            "compressstiffnessscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_compressstiffnessscalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compressstiffnessscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchstiffnessscalemode(
        mut self,
        val: SopWrinkledeformerStretchstiffnessscalemode,
    ) -> Self {
        self.params.insert(
            "stretchstiffnessscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stretchstiffnessscalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchstiffnessscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribsource(mut self, val: SopWrinkledeformerAttribsource) -> Self {
        self.params.insert(
            "attribsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_attribsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_insetscalemode(mut self, val: SopWrinkledeformerInsetscalemode) -> Self {
        self.params.insert(
            "insetscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_insetscalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "insetscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useground(mut self, val: SopWrinkledeformerUseground) -> Self {
        self.params.insert(
            "useground".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_useground_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useground".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dosdfoffset(mut self, val: SopWrinkledeformerDosdfoffset) -> Self {
        self.params.insert(
            "dosdfoffset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dosdfoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dosdfoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wrinklescalemode(mut self, val: SopWrinkledeformerWrinklescalemode) -> Self {
        self.params.insert(
            "wrinklescalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_wrinklescalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wrinklescalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_mode(mut self, val: SopWrinkledeformerVisMode) -> Self {
        self.params.insert(
            "vis_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vis_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pingroup(mut self, val: &str) -> Self {
        self.params.insert(
            "pingroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pingroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pingroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_densityattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "densityattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_densityattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "densityattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thicknessattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "thicknessattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_thicknessattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thicknessattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restlengthscaleattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "restlengthscaleattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restlengthscaleattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restlengthscaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compressstiffnessattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "compressstiffnessattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_compressstiffnessattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compressstiffnessattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchstiffnessattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "stretchstiffnessattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stretchstiffnessattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchstiffnessattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_insetattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "insetattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_insetattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "insetattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_heightfieldpath(mut self, val: &str) -> Self {
        self.params.insert(
            "heightfieldpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_heightfieldpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "heightfieldpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalewrinkleattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "scalewrinkleattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scalewrinkleattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalewrinkleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stressname(mut self, val: &str) -> Self {
        self.params.insert(
            "stressname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stressname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stressname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distancename(mut self, val: &str) -> Self {
        self.params.insert(
            "distancename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_distancename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distancename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rationame(mut self, val: &str) -> Self {
        self.params.insert(
            "rationame".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rationame_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rationame".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_normalcollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "normalcollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normalcollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalcollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scaleignorecollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "scaleignorecollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_scaleignorecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scaleignorecollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updateaffectednmls(mut self, val: bool) -> Self {
        self.params.insert(
            "updateaffectednmls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_updateaffectednmls_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updateaffectednmls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bakestress(mut self, val: bool) -> Self {
        self.params.insert(
            "bakestress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bakestress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakestress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bakedistance(mut self, val: bool) -> Self {
        self.params.insert(
            "bakedistance".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bakedistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakedistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bakeratio(mut self, val: bool) -> Self {
        self.params.insert(
            "bakeratio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bakeratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bakeratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopWrinkledeformer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "wrinkledeformer"
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
