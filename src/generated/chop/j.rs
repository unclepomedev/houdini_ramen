#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopJiggleSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopJiggleUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopJiggle {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopJiggle {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_stiff(mut self, val: f32) -> Self {
        self.params.insert("stiff".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stiff_expr(mut self, expr: &str) -> Self {
        self.params.insert("stiff".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_damp(mut self, val: f32) -> Self {
        self.params.insert("damp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_damp_expr(mut self, expr: &str) -> Self {
        self.params.insert("damp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_limit(mut self, val: f32) -> Self {
        self.params.insert("limit".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_limit_expr(mut self, expr: &str) -> Self {
        self.params.insert("limit".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_flex(mut self, val: f32) -> Self {
        self.params.insert("flex".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_flex_expr(mut self, expr: &str) -> Self {
        self.params.insert("flex".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert("gcolorstep".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert("gcolorstep".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_mult(mut self, val: [f32; 3]) -> Self {
        self.params.insert("mult".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_mult_expr(mut self, expr: &str) -> Self {
        self.params.insert("mult".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("gcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("gcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_srselect(mut self, val: ChopJiggleSrselect) -> Self {
        self.params.insert("srselect".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert("srselect".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_units(mut self, val: ChopJiggleUnits) -> Self {
        self.params.insert("units".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert("units".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_reference(mut self, val: &str) -> Self {
        self.params.insert("reference".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_reference_expr(mut self, expr: &str) -> Self {
        self.params.insert("reference".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert("scope".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert("scope".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert("export".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert("export".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert("timeslice".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert("timeslice".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert("unload".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert("unload".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ChopJiggle {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "jiggle"
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
