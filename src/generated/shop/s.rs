#[derive(Debug, Clone)]
pub struct ShopScalarvolumenoise {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopScalarvolumenoise {
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
    pub fn with_turbulence_freq(mut self, val: f32) -> Self {
        self.params.insert(
            "turbulence_freq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_turbulence_freq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "turbulence_freq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srcmin(mut self, val: f32) -> Self {
        self.params.insert(
            "srcmin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_srcmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srcmax(mut self, val: f32) -> Self {
        self.params.insert(
            "srcmax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_srcmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharpness(mut self, val: f32) -> Self {
        self.params.insert(
            "sharpness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sharpness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharpness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_destmin(mut self, val: f32) -> Self {
        self.params.insert(
            "destmin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_destmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destmin".to_string(),
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
    pub fn with_cell_min(mut self, val: f32) -> Self {
        self.params.insert(
            "cell_min".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cell_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cell_min".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cell_max(mut self, val: f32) -> Self {
        self.params.insert(
            "cell_max".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cell_max_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cell_max".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cell_sharpness(mut self, val: f32) -> Self {
        self.params.insert(
            "cell_sharpness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cell_sharpness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cell_sharpness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cell_influence(mut self, val: f32) -> Self {
        self.params.insert(
            "cell_influence".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cell_influence_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cell_influence".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_voronoi_noise_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "voronoi_noise_seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_voronoi_noise_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "voronoi_noise_seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cell_frequency(mut self, val: f32) -> Self {
        self.params.insert(
            "cell_frequency".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cell_frequency_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cell_frequency".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cell_distance(mut self, val: f32) -> Self {
        self.params.insert(
            "cell_distance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cell_distance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cell_distance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_turbulence_scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "turbulence_scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_turbulence_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "turbulence_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cell_scale(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cell_scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cell_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cell_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_maxoctave(mut self, val: i32) -> Self {
        self.params.insert(
            "maxoctave".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxoctave_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxoctave".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_use_rest(mut self, val: bool) -> Self {
        self.params.insert(
            "use_rest".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_rest_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_rest".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_additive(mut self, val: bool) -> Self {
        self.params.insert(
            "additive".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_additive_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "additive".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_cells(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_cells".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_cells_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_cells".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_change_on_noise(mut self, val: bool) -> Self {
        self.params.insert(
            "change_on_noise".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_change_on_noise_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "change_on_noise".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_invert_cells(mut self, val: bool) -> Self {
        self.params.insert(
            "invert_cells".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_invert_cells_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "invert_cells".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_additive_cells(mut self, val: bool) -> Self {
        self.params.insert(
            "additive_cells".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_additive_cells_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "additive_cells".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clampoutput(mut self, val: bool) -> Self {
        self.params.insert(
            "clampoutput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clampoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clampoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopScalarvolumenoise {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "scalarvolumenoise"
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
pub struct ShopSelect {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopSelect {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_activate1(mut self, val: i32) -> Self {
        self.params.insert(
            "activate1".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activate1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activate1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activate2(mut self, val: i32) -> Self {
        self.params.insert(
            "activate2".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activate2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activate2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activate3(mut self, val: i32) -> Self {
        self.params.insert(
            "activate3".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activate3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activate3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activate4(mut self, val: i32) -> Self {
        self.params.insert(
            "activate4".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activate4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activate4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopSelect {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "select"
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
pub struct ShopSinglesssEval {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopSinglesssEval {
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
    pub fn with_pdf(mut self, val: f32) -> Self {
        self.params.insert(
            "pdf".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pdf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_refl(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refl".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eval(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "eval".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_eval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_bounces(mut self, val: i32) -> Self {
        self.params.insert(
            "bounces".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bounces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mybounces(mut self, val: i32) -> Self {
        self.params.insert(
            "mybounces".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mybounces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mybounces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopSinglesssEval {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "singlesss_eval"
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
pub struct ShopSinglesssSample {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopSinglesssSample {
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
    pub fn with_sx(mut self, val: f32) -> Self {
        self.params
            .insert("sx".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_sx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sy(mut self, val: f32) -> Self {
        self.params
            .insert("sy".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_sy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sy".to_string(),
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
    pub fn with_eta(mut self, val: f32) -> Self {
        self.params.insert(
            "eta".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdf(mut self, val: f32) -> Self {
        self.params.insert(
            "pdf".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pdf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssg(mut self, val: f32) -> Self {
        self.params.insert(
            "ssg".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ssg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ssg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sseta(mut self, val: f32) -> Self {
        self.params.insert(
            "sseta".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sseta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sseta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_u(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("u".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "u".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_absrp(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "absrp".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_absrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "absrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scatr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scatr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scatr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scatr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refl".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssabsrp(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ssabsrp".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ssabsrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ssabsrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssscatr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ssscatr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ssscatr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ssscatr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_bounces(mut self, val: i32) -> Self {
        self.params.insert(
            "bounces".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bounces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bouncetype(mut self, val: i32) -> Self {
        self.params.insert(
            "bouncetype".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bouncetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bouncetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssstype(mut self, val: i32) -> Self {
        self.params.insert(
            "ssstype".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ssstype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ssstype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mybounces(mut self, val: i32) -> Self {
        self.params.insert(
            "mybounces".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mybounces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mybounces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopSinglesssSample {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "singlesss_sample"
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
pub enum ShopSopnetXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopSopnetRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopSopnetPreXform {
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
pub enum ShopSopnetUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ShopSopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopSopnet {
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
    pub fn with_xord(mut self, val: ShopSopnetXord) -> Self {
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
    pub fn with_rord(mut self, val: ShopSopnetRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: ShopSopnetPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: ShopSopnetUparmtype) -> Self {
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

impl crate::core::types::HoudiniNode for ShopSopnet {
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

#[derive(Debug, Clone)]
pub struct ShopSuboutput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopSuboutput {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: ""
    pub fn set_input_input_11<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "" and specifies the output index of the target node.
    pub fn set_input_input_11_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    /// Connects to input 12: ""
    pub fn set_input_input_12<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(12, (target.get_id(), 0));
        self
    }

    /// Connects to input 12: "" and specifies the output index of the target node.
    pub fn set_input_input_12_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(12, (target.get_id(), output_index));
        self
    }

    /// Connects to input 13: ""
    pub fn set_input_input_13<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(13, (target.get_id(), 0));
        self
    }

    /// Connects to input 13: "" and specifies the output index of the target node.
    pub fn set_input_input_13_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(13, (target.get_id(), output_index));
        self
    }

    /// Connects to input 14: ""
    pub fn set_input_input_14<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(14, (target.get_id(), 0));
        self
    }

    /// Connects to input 14: "" and specifies the output index of the target node.
    pub fn set_input_input_14_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(14, (target.get_id(), output_index));
        self
    }

    /// Connects to input 15: ""
    pub fn set_input_input_15<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(15, (target.get_id(), 0));
        self
    }

    /// Connects to input 15: "" and specifies the output index of the target node.
    pub fn set_input_input_15_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(15, (target.get_id(), output_index));
        self
    }

    /// Connects to input 16: ""
    pub fn set_input_input_16<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(16, (target.get_id(), 0));
        self
    }

    /// Connects to input 16: "" and specifies the output index of the target node.
    pub fn set_input_input_16_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(16, (target.get_id(), output_index));
        self
    }

    /// Connects to input 17: ""
    pub fn set_input_input_17<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(17, (target.get_id(), 0));
        self
    }

    /// Connects to input 17: "" and specifies the output index of the target node.
    pub fn set_input_input_17_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(17, (target.get_id(), output_index));
        self
    }

    /// Connects to input 18: ""
    pub fn set_input_input_18<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(18, (target.get_id(), 0));
        self
    }

    /// Connects to input 18: "" and specifies the output index of the target node.
    pub fn set_input_input_18_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(18, (target.get_id(), output_index));
        self
    }

    /// Connects to input 19: ""
    pub fn set_input_input_19<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(19, (target.get_id(), 0));
        self
    }

    /// Connects to input 19: "" and specifies the output index of the target node.
    pub fn set_input_input_19_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(19, (target.get_id(), output_index));
        self
    }

    /// Connects to input 20: ""
    pub fn set_input_input_20<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(20, (target.get_id(), 0));
        self
    }

    /// Connects to input 20: "" and specifies the output index of the target node.
    pub fn set_input_input_20_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(20, (target.get_id(), output_index));
        self
    }

    /// Connects to input 21: ""
    pub fn set_input_input_21<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(21, (target.get_id(), 0));
        self
    }

    /// Connects to input 21: "" and specifies the output index of the target node.
    pub fn set_input_input_21_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(21, (target.get_id(), output_index));
        self
    }

    /// Connects to input 22: ""
    pub fn set_input_input_22<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(22, (target.get_id(), 0));
        self
    }

    /// Connects to input 22: "" and specifies the output index of the target node.
    pub fn set_input_input_22_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(22, (target.get_id(), output_index));
        self
    }

    /// Connects to input 23: ""
    pub fn set_input_input_23<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(23, (target.get_id(), 0));
        self
    }

    /// Connects to input 23: "" and specifies the output index of the target node.
    pub fn set_input_input_23_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(23, (target.get_id(), output_index));
        self
    }

    /// Connects to input 24: ""
    pub fn set_input_input_24<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(24, (target.get_id(), 0));
        self
    }

    /// Connects to input 24: "" and specifies the output index of the target node.
    pub fn set_input_input_24_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(24, (target.get_id(), output_index));
        self
    }

    /// Connects to input 25: ""
    pub fn set_input_input_25<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(25, (target.get_id(), 0));
        self
    }

    /// Connects to input 25: "" and specifies the output index of the target node.
    pub fn set_input_input_25_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(25, (target.get_id(), output_index));
        self
    }

    /// Connects to input 26: ""
    pub fn set_input_input_26<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(26, (target.get_id(), 0));
        self
    }

    /// Connects to input 26: "" and specifies the output index of the target node.
    pub fn set_input_input_26_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(26, (target.get_id(), output_index));
        self
    }
}

impl crate::core::types::HoudiniNode for ShopSuboutput {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "suboutput"
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
pub struct ShopSwitch {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl ShopSwitch {
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
    pub fn with_input(mut self, val: i32) -> Self {
        self.params.insert(
            "input".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_input_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "input".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopSwitch {
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
