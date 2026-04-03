#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopKuwaharafilterMethod {
    Approximate = 0,
    Exact = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopKuwaharafilterLuma {
    SgiLuminance = 0,
    NtscLuminance = 1,
    HdtvLuminance = 2,
    Average = 3,
    MaximumChannel = 4,
    MinimumChannel = 5,
    /// Magnitude / Length
    MagnitudeLength = 6,
    Hue = 7,
    Saturation = 8,
    Value = 9,
    Red = 10,
    Green = 11,
    Blue = 12,
    Alpha = 13,
}

#[derive(Debug, Clone)]
pub struct CopKuwaharafilter {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopKuwaharafilter {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_mask(mut self, val: f32) -> Self {
        self.params.insert("mask".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert("mask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_radius(mut self, val: f32) -> Self {
        self.params.insert("radius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert("radius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blurscale(mut self, val: f32) -> Self {
        self.params.insert("blurscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blurscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("blurscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert("blend".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_separation(mut self, val: f32) -> Self {
        self.params.insert("separation".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_separation_expr(mut self, expr: &str) -> Self {
        self.params.insert("separation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_method(mut self, val: CopKuwaharafilterMethod) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Int(val as i32));
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_luma(mut self, val: CopKuwaharafilterLuma) -> Self {
        self.params.insert("luma".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_luma_expr(mut self, expr: &str) -> Self {
        self.params.insert("luma".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert("signature".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert("signature".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for CopKuwaharafilter {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "kuwaharafilter"
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
