#[derive(Debug, Clone)]
pub struct ShopFresneldiffuseEval {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopFresneldiffuseEval {
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



    // --- Float parameters ---
    pub fn with_pdf(mut self, val: f32) -> Self {
        self.params.insert("pdf".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pdf_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdf".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_eta(mut self, val: f32) -> Self {
        self.params.insert("eta".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_eta_expr(mut self, expr: &str) -> Self {
        self.params.insert("eta".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_u(mut self, val: [f32; 3]) -> Self {
        self.params.insert("u".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_u_expr(mut self, expr: &str) -> Self {
        self.params.insert("u".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert("v".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_v_expr(mut self, expr: &str) -> Self {
        self.params.insert("v".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_refl(mut self, val: [f32; 3]) -> Self {
        self.params.insert("refl".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_refl_expr(mut self, expr: &str) -> Self {
        self.params.insert("refl".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_eval(mut self, val: [f32; 3]) -> Self {
        self.params.insert("eval".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_eval_expr(mut self, expr: &str) -> Self {
        self.params.insert("eval".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_n(mut self, val: [f32; 3]) -> Self {
        self.params.insert("N".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_n_expr(mut self, expr: &str) -> Self {
        self.params.insert("N".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_bounces(mut self, val: i32) -> Self {
        self.params.insert("bounces".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_bounces_expr(mut self, expr: &str) -> Self {
        self.params.insert("bounces".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_reverse(mut self, val: i32) -> Self {
        self.params.insert("reverse".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_reverse_expr(mut self, expr: &str) -> Self {
        self.params.insert("reverse".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mybounces(mut self, val: i32) -> Self {
        self.params.insert("mybounces".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_mybounces_expr(mut self, expr: &str) -> Self {
        self.params.insert("mybounces".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ShopFresneldiffuseEval {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fresneldiffuse_eval"
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
pub struct ShopFresneldiffuseSample {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopFresneldiffuseSample {
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



    // --- Float parameters ---
    pub fn with_sx(mut self, val: f32) -> Self {
        self.params.insert("sx".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_sx_expr(mut self, expr: &str) -> Self {
        self.params.insert("sx".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sy(mut self, val: f32) -> Self {
        self.params.insert("sy".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_sy_expr(mut self, expr: &str) -> Self {
        self.params.insert("sy".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdf(mut self, val: f32) -> Self {
        self.params.insert("pdf".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pdf_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdf".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_eta(mut self, val: f32) -> Self {
        self.params.insert("eta".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_eta_expr(mut self, expr: &str) -> Self {
        self.params.insert("eta".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_u(mut self, val: [f32; 3]) -> Self {
        self.params.insert("u".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_u_expr(mut self, expr: &str) -> Self {
        self.params.insert("u".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_refl(mut self, val: [f32; 3]) -> Self {
        self.params.insert("refl".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_refl_expr(mut self, expr: &str) -> Self {
        self.params.insert("refl".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert("v".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_v_expr(mut self, expr: &str) -> Self {
        self.params.insert("v".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_n(mut self, val: [f32; 3]) -> Self {
        self.params.insert("N".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_n_expr(mut self, expr: &str) -> Self {
        self.params.insert("N".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_flags(mut self, val: i32) -> Self {
        self.params.insert("flags".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_flags_expr(mut self, expr: &str) -> Self {
        self.params.insert("flags".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bounces(mut self, val: i32) -> Self {
        self.params.insert("bounces".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_bounces_expr(mut self, expr: &str) -> Self {
        self.params.insert("bounces".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bouncetype(mut self, val: i32) -> Self {
        self.params.insert("bouncetype".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_bouncetype_expr(mut self, expr: &str) -> Self {
        self.params.insert("bouncetype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mybounces(mut self, val: i32) -> Self {
        self.params.insert("mybounces".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_mybounces_expr(mut self, expr: &str) -> Self {
        self.params.insert("mybounces".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ShopFresneldiffuseSample {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fresneldiffuse_sample"
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
