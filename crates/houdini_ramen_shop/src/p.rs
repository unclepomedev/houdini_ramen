#[derive(Debug, Clone)]
pub struct ShopPhongEval {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ShopPhongEval {
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

    // --- Float parameters ---
    pub fn with_pdf(mut self, val: f32) -> Self {
        self.params.insert(
            "pdf".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pdf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdf".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expx(mut self, val: f32) -> Self {
        self.params.insert(
            "expx".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_expx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expx".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expy(mut self, val: f32) -> Self {
        self.params.insert(
            "expy".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_expy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_u(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "u".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "u".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "v".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refl".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eval(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "eval".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_eval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eval".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aniso_dir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "aniso_dir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_aniso_dir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aniso_dir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_bounces(mut self, val: i32) -> Self {
        self.params.insert(
            "bounces".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bounces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounces".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reverse(mut self, val: i32) -> Self {
        self.params.insert(
            "reverse".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_reverse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reverse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mybounces(mut self, val: i32) -> Self {
        self.params.insert(
            "mybounces".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mybounces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mybounces".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ShopPhongEval {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "phong_eval"
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
pub struct ShopPhongSample {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ShopPhongSample {
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

    // --- Float parameters ---
    pub fn with_sx(mut self, val: f32) -> Self {
        self.params.insert(
            "sx".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sx".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sy(mut self, val: f32) -> Self {
        self.params.insert(
            "sy".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdf(mut self, val: f32) -> Self {
        self.params.insert(
            "pdf".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pdf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdf".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expx(mut self, val: f32) -> Self {
        self.params.insert(
            "expx".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_expx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expx".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expy(mut self, val: f32) -> Self {
        self.params.insert(
            "expy".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_expy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_u(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "u".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "u".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refl".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "v".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aniso_dir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "aniso_dir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_aniso_dir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aniso_dir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_flags(mut self, val: i32) -> Self {
        self.params.insert(
            "flags".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_flags_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flags".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounces(mut self, val: i32) -> Self {
        self.params.insert(
            "bounces".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bounces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounces".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bouncetype(mut self, val: i32) -> Self {
        self.params.insert(
            "bouncetype".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bouncetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bouncetype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mybounces(mut self, val: i32) -> Self {
        self.params.insert(
            "mybounces".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mybounces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mybounces".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ShopPhongSample {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "phong_sample"
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
pub struct ShopPhysicalsssMulti {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ShopPhysicalsssMulti {
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

    // --- Float parameters ---
    pub fn with_g(mut self, val: f32) -> Self {
        self.params.insert(
            "g".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_g_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "g".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eta(mut self, val: f32) -> Self {
        self.params.insert(
            "eta".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eta".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loclradscale(mut self, val: f32) -> Self {
        self.params.insert(
            "loclradscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_loclradscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loclradscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loclquality(mut self, val: f32) -> Self {
        self.params.insert(
            "loclquality".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_loclquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loclquality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_globquality(mut self, val: f32) -> Self {
        self.params.insert(
            "globquality".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_globquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "globquality".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcautofactor(mut self, val: f32) -> Self {
        self.params.insert(
            "pcautofactor".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pcautofactor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcautofactor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcerror(mut self, val: f32) -> Self {
        self.params.insert(
            "pcerror".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pcerror_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcerror".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_absrp(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "absrp".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_absrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "absrp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scatr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scatr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scatr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scatr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_multiloclclr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "multiloclclr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_multiloclclr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "multiloclclr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_multiglobclr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "multiglobclr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_multiglobclr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "multiglobclr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_optsecond(mut self, val: i32) -> Self {
        self.params.insert(
            "optsecond".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_optsecond_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "optsecond".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcmode(mut self, val: i32) -> Self {
        self.params.insert(
            "pcmode".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pcmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcsavepos(mut self, val: i32) -> Self {
        self.params.insert(
            "pcsavepos".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pcsavepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcsavepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcsaveirrad(mut self, val: i32) -> Self {
        self.params.insert(
            "pcsaveirrad".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pcsaveirrad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcsaveirrad".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcautosize(mut self, val: i32) -> Self {
        self.params.insert(
            "pcautosize".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pcautosize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcautosize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcsize(mut self, val: i32) -> Self {
        self.params.insert(
            "pcsize".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pcsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcfilter(mut self, val: i32) -> Self {
        self.params.insert(
            "pcfilter".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pcfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcfilter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcfiltersize(mut self, val: i32) -> Self {
        self.params.insert(
            "pcfiltersize".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pcfiltersize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcfiltersize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spectral(mut self, val: i32) -> Self {
        self.params.insert(
            "spectral".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_spectral_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spectral".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_multimodel(mut self, val: &str) -> Self {
        self.params.insert(
            "multimodel".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_multimodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "multimodel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcname(mut self, val: &str) -> Self {
        self.params.insert(
            "pcname".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pcname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ShopPhysicalsssMulti {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "physicalsss_multi"
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
pub struct ShopPhysicalsssSingle {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ShopPhysicalsssSingle {
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

    // --- Float parameters ---
    pub fn with_g(mut self, val: f32) -> Self {
        self.params.insert(
            "g".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_g_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "g".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eta(mut self, val: f32) -> Self {
        self.params.insert(
            "eta".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eta".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_absrp(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "absrp".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_absrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "absrp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scatr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scatr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scatr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scatr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_optsecond(mut self, val: i32) -> Self {
        self.params.insert(
            "optsecond".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_optsecond_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "optsecond".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "nsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_nsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spectral(mut self, val: i32) -> Self {
        self.params.insert(
            "spectral".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_spectral_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spectral".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ShopPhysicalsssSingle {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "physicalsss_single"
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
pub struct ShopPrincipleddiffuseEval {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ShopPrincipleddiffuseEval {
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

    // --- Float parameters ---
    pub fn with_pdf(mut self, val: f32) -> Self {
        self.params.insert(
            "pdf".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pdf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdf".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subsurface(mut self, val: f32) -> Self {
        self.params.insert(
            "subsurface".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_subsurface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsurface".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_u(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "u".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "u".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "v".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refl".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eval(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "eval".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_eval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eval".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ng(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ng".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ng_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ng".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "baseColor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_basecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "baseColor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sheen(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sheen".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sheen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sheen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_bounces(mut self, val: i32) -> Self {
        self.params.insert(
            "bounces".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bounces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounces".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reverse(mut self, val: i32) -> Self {
        self.params.insert(
            "reverse".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_reverse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reverse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mybounces(mut self, val: i32) -> Self {
        self.params.insert(
            "mybounces".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mybounces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mybounces".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ShopPrincipleddiffuseEval {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "principleddiffuse_eval"
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
pub struct ShopPrincipleddiffuseSample {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ShopPrincipleddiffuseSample {
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

    // --- Float parameters ---
    pub fn with_sx(mut self, val: f32) -> Self {
        self.params.insert(
            "sx".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sx".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sy(mut self, val: f32) -> Self {
        self.params.insert(
            "sy".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdf(mut self, val: f32) -> Self {
        self.params.insert(
            "pdf".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pdf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdf".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subsurface(mut self, val: f32) -> Self {
        self.params.insert(
            "subsurface".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_subsurface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsurface".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_u(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "u".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "u".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refl".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "v".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ng(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ng".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ng_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ng".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "baseColor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_basecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "baseColor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sheen(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sheen".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sheen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sheen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_flags(mut self, val: i32) -> Self {
        self.params.insert(
            "flags".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_flags_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flags".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounces(mut self, val: i32) -> Self {
        self.params.insert(
            "bounces".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bounces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounces".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bouncetype(mut self, val: i32) -> Self {
        self.params.insert(
            "bouncetype".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bouncetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bouncetype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mybounces(mut self, val: i32) -> Self {
        self.params.insert(
            "mybounces".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mybounces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mybounces".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ShopPrincipleddiffuseSample {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "principleddiffuse_sample"
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
pub enum ShopPrincipledshaderMetallicMonochannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderReflectMonochannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderReflecttintMonochannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderRoughMonochannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderAnisoMonochannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderAnisodirMonochannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderSubsurfaceMonochannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderSheenMonochannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderSheentintMonochannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderCoatMonochannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderCoatroughMonochannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderNormaltexchannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderNormaltexnormalspace {
    /// 0 to 1
    N0To1 = 0,
    /// -1 to 1
    Minus1To1 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderCoatnormaltexchannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderCoatnormaltexnormalspace {
    /// 0 to 1
    N0To1 = 0,
    /// -1 to 1
    Minus1To1 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderDisptexchannel {
    Luminance = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderBakeNormalspace {
    /// 0 to 1
    N0To1 = 0,
    /// -1 to 1
    Minus1To1 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopPrincipledshaderOglRoughmapComp {
    Red = 0,
    Green = 1,
    Blue = 2,
    Alpha = 3,
}

#[derive(Debug, Clone)]
pub struct ShopPrincipledshader {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ShopPrincipledshader {
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

    // --- Float parameters ---
    pub fn with_basecolor_textureintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "basecolor_textureIntensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_basecolor_textureintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecolor_textureIntensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflect(mut self, val: f32) -> Self {
        self.params.insert(
            "reflect".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflecttint(mut self, val: f32) -> Self {
        self.params.insert(
            "reflecttint".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflecttint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflecttint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
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
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aniso(mut self, val: f32) -> Self {
        self.params.insert(
            "aniso".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aniso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aniso".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisodir(mut self, val: f32) -> Self {
        self.params.insert(
            "anisodir".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anisodir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisodir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subsurface(mut self, val: f32) -> Self {
        self.params.insert(
            "subsurface".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_subsurface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsurface".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sheen(mut self, val: f32) -> Self {
        self.params.insert(
            "sheen".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sheen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sheen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sheentint(mut self, val: f32) -> Self {
        self.params.insert(
            "sheentint".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sheentint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sheentint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coat(mut self, val: f32) -> Self {
        self.params.insert(
            "coat".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
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
    pub fn with_coatrough(mut self, val: f32) -> Self {
        self.params.insert(
            "coatrough".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coatrough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatrough".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_texturefilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_textureFilterWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_texturefilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_textureFilterWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emitint(mut self, val: f32) -> Self {
        self.params.insert(
            "emitint".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_emitint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emitint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emitcolor_textureintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "emitcolor_textureIntensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_emitcolor_textureintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emitcolor_textureIntensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexscale(mut self, val: f32) -> Self {
        self.params.insert(
            "normalTexScale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normaltexscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexScale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexfilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "normalTexFilterWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normaltexfilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexFilterWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexscale(mut self, val: f32) -> Self {
        self.params.insert(
            "coatNormalTexScale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coatnormaltexscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexScale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexfilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "coatNormalTexFilterWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coatnormaltexfilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexFilterWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_displacebound(mut self, val: f32) -> Self {
        self.params.insert(
            "vm_displacebound".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vm_displacebound_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_displacebound".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "dispTexOffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_disptexoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexOffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexscale(mut self, val: f32) -> Self {
        self.params.insert(
            "dispTexScale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_disptexscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexScale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexfilterwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "dispTexfilterWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_disptexfilterwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexfilterWidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dispnoiseamp(mut self, val: f32) -> Self {
        self.params.insert(
            "dispNoiseAmp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dispnoiseamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispNoiseAmp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dispnoiserough(mut self, val: f32) -> Self {
        self.params.insert(
            "dispNoiseRough".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dispnoiserough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispNoiseRough".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dispnoiseatten(mut self, val: f32) -> Self {
        self.params.insert(
            "dispNoiseAtten".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dispnoiseatten_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispNoiseAtten".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_occlusionbias(mut self, val: f32) -> Self {
        self.params.insert(
            "bake_occlusionbias".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bake_occlusionbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_occlusionbias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_cavitydistance(mut self, val: f32) -> Self {
        self.params.insert(
            "bake_cavitydistance".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bake_cavitydistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_cavitydistance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_cavitybias(mut self, val: f32) -> Self {
        self.params.insert(
            "bake_cavitybias".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bake_cavitybias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_cavitybias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_curvaturescale(mut self, val: f32) -> Self {
        self.params.insert(
            "bake_curvaturescale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bake_curvaturescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_curvaturescale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_rough".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_rough".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_ior_inner(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_ior_inner".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_ior_inner_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_ior_inner".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_ior_outer(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_ior_outer".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_ior_outer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_ior_outer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpscale(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_bumpscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_bumpscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumpscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_displacescale(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_displacescale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_displacescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_displacescale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_displaceoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_displaceoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_displaceoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_displaceoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_envscale(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_envscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_envscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_envscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_curvaturebias(mut self, val: f32) -> Self {
        self.params.insert(
            "bake_curvaturebias".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bake_curvaturebias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_curvaturebias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_ogl_shinyrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "ogl_shinyrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ogl_shinyrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_shinyrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_basecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "basecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_basecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emitcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "emitcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_emitcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emitcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dispnoisefreq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dispNoiseFreq".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dispnoisefreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispNoiseFreq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dispnoiseoffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dispNoiseOffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dispnoiseoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispNoiseOffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_amb(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_amb".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_amb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_amb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_emit(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_emit".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_emit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_emit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_diff(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_diff".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_diff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_diff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_spec(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_spec".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_spec".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_envrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_envrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_envrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_envrotate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_metallic_monochannel(
        mut self,
        val: ShopPrincipledshaderMetallicMonochannel,
    ) -> Self {
        self.params.insert(
            "metallic_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_metallic_monochannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "metallic_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflect_monochannel(mut self, val: ShopPrincipledshaderReflectMonochannel) -> Self {
        self.params.insert(
            "reflect_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_reflect_monochannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflect_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflecttint_monochannel(
        mut self,
        val: ShopPrincipledshaderReflecttintMonochannel,
    ) -> Self {
        self.params.insert(
            "reflecttint_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_reflecttint_monochannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflecttint_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rough_monochannel(mut self, val: ShopPrincipledshaderRoughMonochannel) -> Self {
        self.params.insert(
            "rough_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_rough_monochannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rough_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aniso_monochannel(mut self, val: ShopPrincipledshaderAnisoMonochannel) -> Self {
        self.params.insert(
            "aniso_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_aniso_monochannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aniso_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisodir_monochannel(
        mut self,
        val: ShopPrincipledshaderAnisodirMonochannel,
    ) -> Self {
        self.params.insert(
            "anisodir_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_anisodir_monochannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisodir_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subsurface_monochannel(
        mut self,
        val: ShopPrincipledshaderSubsurfaceMonochannel,
    ) -> Self {
        self.params.insert(
            "subsurface_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_subsurface_monochannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsurface_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sheen_monochannel(mut self, val: ShopPrincipledshaderSheenMonochannel) -> Self {
        self.params.insert(
            "sheen_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_sheen_monochannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sheen_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sheentint_monochannel(
        mut self,
        val: ShopPrincipledshaderSheentintMonochannel,
    ) -> Self {
        self.params.insert(
            "sheentint_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_sheentint_monochannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sheentint_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coat_monochannel(mut self, val: ShopPrincipledshaderCoatMonochannel) -> Self {
        self.params.insert(
            "coat_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_coat_monochannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coat_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatrough_monochannel(
        mut self,
        val: ShopPrincipledshaderCoatroughMonochannel,
    ) -> Self {
        self.params.insert(
            "coatrough_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_coatrough_monochannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatrough_monoChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexchannel(mut self, val: ShopPrincipledshaderNormaltexchannel) -> Self {
        self.params.insert(
            "normalTexChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_normaltexchannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexnormalspace(
        mut self,
        val: ShopPrincipledshaderNormaltexnormalspace,
    ) -> Self {
        self.params.insert(
            "normalTexNormalSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_normaltexnormalspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexNormalSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexchannel(
        mut self,
        val: ShopPrincipledshaderCoatnormaltexchannel,
    ) -> Self {
        self.params.insert(
            "coatNormalTexChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_coatnormaltexchannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexnormalspace(
        mut self,
        val: ShopPrincipledshaderCoatnormaltexnormalspace,
    ) -> Self {
        self.params.insert(
            "coatNormalTexNormalSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_coatnormaltexnormalspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexNormalSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexchannel(mut self, val: ShopPrincipledshaderDisptexchannel) -> Self {
        self.params.insert(
            "dispTexChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_disptexchannel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexChannel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dispnoiseturb(mut self, val: i32) -> Self {
        self.params.insert(
            "dispNoiseTurb".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_dispnoiseturb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispNoiseTurb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "bake_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bake_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_samples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_normalspace(mut self, val: ShopPrincipledshaderBakeNormalspace) -> Self {
        self.params.insert(
            "bake_normalspace".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_bake_normalspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_normalspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_speclayer(mut self, val: i32) -> Self {
        self.params.insert(
            "ogl_speclayer".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ogl_speclayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_speclayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_roughmap_comp(mut self, val: ShopPrincipledshaderOglRoughmapComp) -> Self {
        self.params.insert(
            "ogl_roughmap_comp".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_ogl_roughmap_comp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_roughmap_comp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_opacitylayer(mut self, val: i32) -> Self {
        self.params.insert(
            "ogl_opacitylayer".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ogl_opacitylayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_opacitylayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumplayer(mut self, val: i32) -> Self {
        self.params.insert(
            "ogl_bumplayer".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ogl_bumplayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumplayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_normallayer(mut self, val: i32) -> Self {
        self.params.insert(
            "ogl_normallayer".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ogl_normallayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normallayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_basecolor_texture(mut self, val: &str) -> Self {
        self.params.insert(
            "basecolor_texture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_basecolor_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecolor_texture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basecolor_texturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "basecolor_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_basecolor_texturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecolor_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basecolor_texturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "basecolor_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_basecolor_texturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecolor_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_metallic_texture(mut self, val: &str) -> Self {
        self.params.insert(
            "metallic_texture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_metallic_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "metallic_texture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_metallic_texturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "metallic_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_metallic_texturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "metallic_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_metallic_texturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "metallic_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_metallic_texturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "metallic_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflect_texture(mut self, val: &str) -> Self {
        self.params.insert(
            "reflect_texture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reflect_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflect_texture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflect_texturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "reflect_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reflect_texturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflect_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflect_texturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "reflect_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reflect_texturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflect_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflecttint_texture(mut self, val: &str) -> Self {
        self.params.insert(
            "reflecttint_texture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reflecttint_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflecttint_texture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflecttint_texturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "reflecttint_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reflecttint_texturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflecttint_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflecttint_texturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "reflecttint_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reflecttint_texturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflecttint_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rough_texture(mut self, val: &str) -> Self {
        self.params.insert(
            "rough_texture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rough_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rough_texture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rough_texturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "rough_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rough_texturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rough_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rough_texturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "rough_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rough_texturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rough_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aniso_texture(mut self, val: &str) -> Self {
        self.params.insert(
            "aniso_texture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aniso_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aniso_texture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aniso_texturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "aniso_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aniso_texturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aniso_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aniso_texturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "aniso_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aniso_texturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aniso_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisodir_texture(mut self, val: &str) -> Self {
        self.params.insert(
            "anisodir_texture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_anisodir_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisodir_texture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisodir_texturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "anisodir_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_anisodir_texturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisodir_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisodir_texturefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "anisodir_textureFilter".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_anisodir_texturefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisodir_textureFilter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisodir_texturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "anisodir_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_anisodir_texturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisodir_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subsurface_texture(mut self, val: &str) -> Self {
        self.params.insert(
            "subsurface_texture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subsurface_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsurface_texture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subsurface_texturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "subsurface_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subsurface_texturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsurface_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subsurface_texturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "subsurface_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subsurface_texturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsurface_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sheen_texture(mut self, val: &str) -> Self {
        self.params.insert(
            "sheen_texture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sheen_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sheen_texture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sheen_texturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "sheen_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sheen_texturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sheen_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sheen_texturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "sheen_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sheen_texturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sheen_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sheentint_texture(mut self, val: &str) -> Self {
        self.params.insert(
            "sheentint_texture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sheentint_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sheentint_texture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sheentint_texturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "sheentint_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sheentint_texturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sheentint_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sheentint_texturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "sheentint_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sheentint_texturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sheentint_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coat_texture(mut self, val: &str) -> Self {
        self.params.insert(
            "coat_texture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coat_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coat_texture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coat_texturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "coat_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coat_texturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coat_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coat_texturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "coat_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coat_texturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coat_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatrough_texture(mut self, val: &str) -> Self {
        self.params.insert(
            "coatrough_texture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatrough_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatrough_texture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatrough_texturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "coatrough_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatrough_texturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatrough_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatrough_texturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "coatrough_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatrough_texturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatrough_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basecolor_texturefilter(mut self, val: &str) -> Self {
        self.params.insert(
            "basecolor_textureFilter".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_basecolor_texturefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecolor_textureFilter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emitcolor_texture(mut self, val: &str) -> Self {
        self.params.insert(
            "emitcolor_texture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_emitcolor_texture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emitcolor_texture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emitcolor_texturewrap(mut self, val: &str) -> Self {
        self.params.insert(
            "emitcolor_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_emitcolor_texturewrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emitcolor_textureWrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emitcolor_texturecolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "emitcolor_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_emitcolor_texturecolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emitcolor_textureColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltextype(mut self, val: &str) -> Self {
        self.params.insert(
            "normalTexType".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normaltextype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexType".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexcolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "normalTexColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normaltexcolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexvectorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "normalTexVectorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normaltexvectorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexVectorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexture(mut self, val: &str) -> Self {
        self.params.insert(
            "normalTexture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normaltexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexwrap(mut self, val: &str) -> Self {
        self.params.insert(
            "normalTexWrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normaltexwrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexWrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "normalTexFilter".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normaltexfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexFilter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normalteximageplane(mut self, val: &str) -> Self {
        self.params.insert(
            "normalTexImagePlane".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normalteximageplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexImagePlane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltextype(mut self, val: &str) -> Self {
        self.params.insert(
            "coatNormalTexType".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatnormaltextype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexType".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexcolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "coatNormalTexColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexcolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexvectorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "coatNormalTexVectorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexvectorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexVectorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexture(mut self, val: &str) -> Self {
        self.params.insert(
            "coatNormalTexture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexwrap(mut self, val: &str) -> Self {
        self.params.insert(
            "coatNormalTexWrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexwrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexWrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "coatNormalTexFilter".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexFilter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormalteximageplane(mut self, val: &str) -> Self {
        self.params.insert(
            "coatNormalTexImagePlane".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatnormalteximageplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexImagePlane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptextype(mut self, val: &str) -> Self {
        self.params.insert(
            "dispTexType".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_disptextype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexType".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexcolorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "dispTexColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_disptexcolorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexColorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexvectorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "dispTexVectorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_disptexvectorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexVectorSpace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptextexture(mut self, val: &str) -> Self {
        self.params.insert(
            "dispTexTexture".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_disptextexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexwrap(mut self, val: &str) -> Self {
        self.params.insert(
            "dispTexWrap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_disptexwrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexWrap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disptexfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "dispTexFilter".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_disptexfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispTexFilter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dispnoisetype(mut self, val: &str) -> Self {
        self.params.insert(
            "dispNoiseType".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dispnoisetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dispNoiseType".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_difflabel(mut self, val: &str) -> Self {
        self.params.insert(
            "difflabel".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_difflabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "difflabel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_baselabel(mut self, val: &str) -> Self {
        self.params.insert(
            "baselabel".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_baselabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "baselabel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatlabel(mut self, val: &str) -> Self {
        self.params.insert(
            "coatlabel".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coatlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ogl_tex{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_min_filter_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_min_filter{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_min_filter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_min_filter{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_mag_filter_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_mag_filter{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_mag_filter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_mag_filter{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_spec_model(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_spec_model".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_spec_model_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_spec_model".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_specmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_specmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_specmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_specmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_roughmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_roughmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_roughmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_roughmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_opacitymap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_opacitymap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_opacitymap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_opacitymap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_bumpmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumpmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumptype(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_bumptype".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_bumptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumptype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpbias(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_bumpbias".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumpbias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_normalmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_normalmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_normalmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normalmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_normalmap_type(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_normalmap_type".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_normalmap_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normalmap_type".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_normalbias(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_normalbias".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_normalbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normalbias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_displacemap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_displacemap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_displacemap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_displacemap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_envmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_envmap".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_envmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_envmap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_envrotorder(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_envrotorder".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_envrotorder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_envrotorder".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_basecolor_usepointcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "basecolor_usePointColor".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_basecolor_usepointcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecolor_usePointColor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basecolor_usepackedcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "basecolor_usePackedColor".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_basecolor_usepackedcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecolor_usePackedColor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basecolor_usetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "basecolor_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_basecolor_usetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecolor_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_metallic_usetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "metallic_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_metallic_usetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "metallic_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflect_usetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "reflect_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reflect_usetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflect_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflecttint_usetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "reflecttint_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reflecttint_usetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflecttint_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rough_usetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "rough_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rough_usetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rough_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aniso_usetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "aniso_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aniso_usetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aniso_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisodir_usetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "anisodir_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_anisodir_usetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisodir_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subsurface_usetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "subsurface_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_subsurface_usetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsurface_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sheen_usetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "sheen_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sheen_usetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sheen_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sheentint_usetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "sheentint_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sheentint_usetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sheentint_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coat_usetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "coat_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coat_usetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coat_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatrough_usetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "coatrough_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coatrough_usetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatrough_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emitillum(mut self, val: bool) -> Self {
        self.params.insert(
            "emitillum".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_emitillum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emitillum".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emitcolor_usetexture(mut self, val: bool) -> Self {
        self.params.insert(
            "emitcolor_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_emitcolor_usetexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emitcolor_useTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablebumpornormaltexture(mut self, val: bool) -> Self {
        self.params.insert(
            "enableBumpOrNormalTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablebumpornormaltexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableBumpOrNormalTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexudim(mut self, val: bool) -> Self {
        self.params.insert(
            "normalTexUdim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normaltexudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexUdim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexnormalflipx(mut self, val: bool) -> Self {
        self.params.insert(
            "normalTexNormalFlipX".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normaltexnormalflipx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexNormalFlipX".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaltexnormalflipy(mut self, val: bool) -> Self {
        self.params.insert(
            "normalTexNormalFlipY".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normaltexnormalflipy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalTexNormalFlipY".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_separatecoatnormals(mut self, val: bool) -> Self {
        self.params.insert(
            "separateCoatNormals".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_separatecoatnormals_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "separateCoatNormals".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecoatnormaltexture(mut self, val: bool) -> Self {
        self.params.insert(
            "enableCoatNormalTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecoatnormaltexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableCoatNormalTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexudim(mut self, val: bool) -> Self {
        self.params.insert(
            "coatNormalTexUdim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coatnormaltexudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexUdim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexnormalflipx(mut self, val: bool) -> Self {
        self.params.insert(
            "coatNormalTexNormalFlipX".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coatnormaltexnormalflipx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexNormalFlipX".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatnormaltexnormalflipy(mut self, val: bool) -> Self {
        self.params.insert(
            "coatNormalTexNormalFlipY".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_coatnormaltexnormalflipy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatNormalTexNormalFlipY".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shop_disable_displace_shader(mut self, val: bool) -> Self {
        self.params.insert(
            "shop_disable_displace_shader".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shop_disable_displace_shader_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_disable_displace_shader".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vm_truedisplace(mut self, val: bool) -> Self {
        self.params.insert(
            "vm_truedisplace".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vm_truedisplace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vm_truedisplace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledisptexture(mut self, val: bool) -> Self {
        self.params.insert(
            "enableDispTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledisptexture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableDispTexture".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledispnoise(mut self, val: bool) -> Self {
        self.params.insert(
            "enableDispNoise".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledispnoise_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableDispNoise".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_tangentnormalflipx(mut self, val: bool) -> Self {
        self.params.insert(
            "bake_tangentnormalflipx".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bake_tangentnormalflipx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_tangentnormalflipx".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_tangentnormalflipy(mut self, val: bool) -> Self {
        self.params.insert(
            "bake_tangentnormalflipy".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bake_tangentnormalflipy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_tangentnormalflipy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bake_include_disp_nt(mut self, val: bool) -> Self {
        self.params.insert(
            "bake_include_disp_nt".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bake_include_disp_nt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bake_include_disp_nt".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_udim(mut self, val: bool) -> Self {
        self.params.insert(
            "udim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_udim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "udim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_light(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_light".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_light_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_light".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_cutout(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_cutout".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_cutout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_cutout".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_clamping_mode_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("ogl_clamping_mode{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_clamping_mode_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_clamping_mode{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_adjustshiny(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_adjustshiny".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_adjustshiny_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_adjustshiny".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_bumpinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_bumpinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumpinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_normalflipx(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_normalflipx".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_normalflipx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normalflipx".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_normalflipy(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_normalflipy".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_normalflipy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normalflipy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ShopPrincipledshader {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "principledshader"
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
pub struct ShopProperties {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl ShopProperties {
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
}

impl houdini_ramen_core::types::HoudiniNode for ShopProperties {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "properties"
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
pub struct ShopPyro {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ShopPyro {
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
    pub fn trigger_pcgen_execute(mut self) -> Self {
        self.params.insert(
            "pcgen_execute".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_s_fieldk(mut self, val: f32) -> Self {
        self.params.insert(
            "s_fieldk".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_s_fieldk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_fieldk".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_alb(mut self, val: f32) -> Self {
        self.params.insert(
            "s_alb".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_s_alb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_alb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_shadk(mut self, val: f32) -> Self {
        self.params.insert(
            "s_shadk".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_s_shadk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_shadk".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_amp(mut self, val: f32) -> Self {
        self.params.insert(
            "b_amp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_b_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_amp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_kdens(mut self, val: f32) -> Self {
        self.params.insert(
            "b_kdens".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_b_kdens_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_kdens".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "b_blend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_b_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_blend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bph_temp(mut self, val: f32) -> Self {
        self.params.insert(
            "bph_temp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bph_temp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bph_temp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bph_kcool(mut self, val: f32) -> Self {
        self.params.insert(
            "bph_kcool".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bph_kcool_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bph_kcool".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bph_bshift(mut self, val: f32) -> Self {
        self.params.insert(
            "bph_bshift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bph_bshift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bph_bshift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bph_ksofte(mut self, val: f32) -> Self {
        self.params.insert(
            "bph_ksofte".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bph_ksofte_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bph_ksofte".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bph_ksoftk(mut self, val: f32) -> Self {
        self.params.insert(
            "bph_ksoftk".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bph_ksoftk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bph_ksoftk".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcgen_steps(mut self, val: f32) -> Self {
        self.params.insert(
            "pcgen_steps".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pcgen_steps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcgen_steps".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcgen_stepf(mut self, val: f32) -> Self {
        self.params.insert(
            "pcgen_stepf".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pcgen_stepf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcgen_stepf".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_amp(mut self, val: f32) -> Self {
        self.params.insert(
            "ss_amp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ss_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_amp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_attdens(mut self, val: f32) -> Self {
        self.params.insert(
            "ss_attdens".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ss_attdens_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_attdens".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_attdist(mut self, val: f32) -> Self {
        self.params.insert(
            "ss_attdist".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ss_attdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_attdist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_phase(mut self, val: f32) -> Self {
        self.params.insert(
            "ss_phase".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ss_phase_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_phase".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_rad(mut self, val: f32) -> Self {
        self.params.insert(
            "ss_rad".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ss_rad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_rad".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_shqual(mut self, val: f32) -> Self {
        self.params.insert(
            "ss_shqual".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ss_shqual_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_shqual".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_sharpe1(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_sharpe1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_sharpe1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_sharpe1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_sharpk1(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_sharpk1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_sharpk1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_sharpk1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_softe1(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_softe1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_softe1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_softe1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_softk1(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_softk1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_softk1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_softk1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngsl1(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngsl1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngsl1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngsl1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngsh1(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngsh1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngsh1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngsh1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngtl1(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngtl1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngtl1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngtl1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngth1(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngth1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngth1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngth1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_sharpe2(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_sharpe2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_sharpe2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_sharpe2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_sharpk2(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_sharpk2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_sharpk2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_sharpk2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_softe2(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_softe2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_softe2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_softe2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_softk2(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_softk2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_softk2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_softk2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngsl2(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngsl2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngsl2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngsl2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngsh2(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngsh2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngsh2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngsh2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngtl2(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngtl2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngtl2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngtl2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngth2(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngth2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngth2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngth2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_sharpe3(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_sharpe3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_sharpe3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_sharpe3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_sharpk3(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_sharpk3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_sharpk3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_sharpk3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_softe3(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_softe3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_softe3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_softe3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_softk3(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_softk3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_softk3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_softk3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngsl3(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngsl3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngsl3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngsl3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngsh3(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngsh3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngsh3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngsh3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngtl3(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngtl3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngtl3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngtl3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngth3(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngth3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngth3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngth3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_sharpe4(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_sharpe4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_sharpe4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_sharpe4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_sharpk4(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_sharpk4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_sharpk4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_sharpk4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_softe4(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_softe4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_softe4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_softe4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_softk4(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_softk4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_softk4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_softk4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngsl4(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngsl4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngsl4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngsl4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngsh4(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngsh4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngsh4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngsh4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngtl4(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngtl4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngtl4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngtl4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngth4(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngth4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngth4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngth4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_sharpe5(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_sharpe5".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_sharpe5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_sharpe5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_sharpk5(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_sharpk5".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_sharpk5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_sharpk5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_softe5(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_softe5".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_softe5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_softe5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_softk5(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_softk5".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_softk5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_softk5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngsl5(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngsl5".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngsl5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngsl5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngsh5(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngsh5".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngsh5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngsh5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngtl5(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngtl5".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngtl5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngtl5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngth5(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngth5".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngth5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngth5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_sharpe6(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_sharpe6".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_sharpe6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_sharpe6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_sharpk6(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_sharpk6".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_sharpk6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_sharpk6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_softe6(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_softe6".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_softe6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_softe6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_softk6(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_softk6".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_softk6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_softk6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngsl6(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngsl6".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngsl6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngsl6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngsh6(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngsh6".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngsh6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngsh6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngtl6(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngtl6".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngtl6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngtl6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngth6(mut self, val: f32) -> Self {
        self.params.insert(
            "mf_rngth6".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mf_rngth6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngth6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngsl1(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_rngsl1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_rngsl1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngsl1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngsh1(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_rngsh1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_rngsh1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngsh1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_fw1(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_fw1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_fw1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_fw1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_expon1(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_expon1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_expon1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_expon1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_flamp1(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_flamp1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_flamp1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_flamp1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_oct1(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_oct1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_oct1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_oct1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_lac1(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_lac1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_lac1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_lac1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_gain1(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_gain1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_gain1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_gain1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_disp1(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_disp1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_disp1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_disp1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngol1(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_rngol1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_rngol1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngol1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngoh1(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_rngoh1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_rngoh1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngoh1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_amp1(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_amp1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_amp1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_amp1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngsl2(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_rngsl2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_rngsl2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngsl2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngsh2(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_rngsh2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_rngsh2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngsh2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_fw2(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_fw2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_fw2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_fw2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_expon2(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_expon2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_expon2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_expon2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_flamp2(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_flamp2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_flamp2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_flamp2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_oct2(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_oct2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_oct2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_oct2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_lac2(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_lac2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_lac2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_lac2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_gain2(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_gain2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_gain2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_gain2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_disp2(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_disp2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_disp2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_disp2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngol2(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_rngol2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_rngol2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngol2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngoh2(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_rngoh2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_rngoh2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngoh2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_amp2(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_amp2".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_amp2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_amp2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngsl3(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_rngsl3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_rngsl3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngsl3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngsh3(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_rngsh3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_rngsh3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngsh3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_fw3(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_fw3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_fw3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_fw3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_expon3(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_expon3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_expon3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_expon3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_flamp3(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_flamp3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_flamp3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_flamp3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_oct3(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_oct3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_oct3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_oct3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_lac3(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_lac3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_lac3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_lac3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_gain3(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_gain3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_gain3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_gain3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_disp3(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_disp3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_disp3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_disp3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngol3(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_rngol3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_rngol3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngol3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngoh3(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_rngoh3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_rngoh3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngoh3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_amp3(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_amp3".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_amp3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_amp3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngsl4(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_rngsl4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_rngsl4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngsl4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngsh4(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_rngsh4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_rngsh4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngsh4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_fw4(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_fw4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_fw4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_fw4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_expon4(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_expon4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_expon4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_expon4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_flamp4(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_flamp4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_flamp4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_flamp4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_oct4(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_oct4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_oct4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_oct4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_lac4(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_lac4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_lac4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_lac4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_gain4(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_gain4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_gain4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_gain4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_disp4(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_disp4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_disp4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_disp4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngol4(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_rngol4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_rngol4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngol4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngoh4(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_rngoh4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_rngoh4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngoh4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_amp4(mut self, val: f32) -> Self {
        self.params.insert(
            "mn_amp4".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mn_amp4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_amp4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_time(mut self, val: f32) -> Self {
        self.params.insert(
            "time".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_time_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "time".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_frame(mut self, val: f32) -> Self {
        self.params.insert(
            "frame".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frame_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frame".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dr_start(mut self, val: f32) -> Self {
        self.params.insert(
            "dr_start".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dr_start_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dr_start".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dr_rate(mut self, val: f32) -> Self {
        self.params.insert(
            "dr_rate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dr_rate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dr_rate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_s_hue(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s_hue".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_hue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_hue".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_shadh(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s_shadh".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_shadh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_shadh".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_phase(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s_phase".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_phase_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_phase".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_cchsv(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s_cchsv".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_cchsv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_cchsv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_cccont(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s_cccont".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_cccont_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_cccont".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_ccgamma(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s_ccgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_ccgamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_ccgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_cctint(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s_cctint".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_cctint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_cctint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_cchsv(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "b_cchsv".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_b_cchsv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_cchsv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_cccont(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "b_cccont".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_b_cccont_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_cccont".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_ccgamma(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "b_ccgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_b_ccgamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_ccgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_cctint(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "b_cctint".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_b_cctint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_cctint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_cchsv(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ss_cchsv".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ss_cchsv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_cchsv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_cccont(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ss_cccont".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ss_cccont_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_cccont".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_ccgamma(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ss_ccgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ss_ccgamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_ccgamma".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_cctint(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ss_cctint".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ss_cctint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_cctint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_mn_freq1(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "mn_freq1".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_mn_freq1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_freq1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_off1(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "mn_off1".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_mn_off1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_off1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_freq2(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "mn_freq2".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_mn_freq2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_freq2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_off2(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "mn_off2".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_mn_off2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_off2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_freq3(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "mn_freq3".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_mn_freq3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_freq3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_off3(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "mn_off3".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_mn_off3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_off3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_freq4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "mn_freq4".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_mn_freq4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_freq4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_off4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "mn_off4".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_mn_off4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_off4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pcgen_frng1(mut self, val: i32) -> Self {
        self.params.insert(
            "pcgen_frng1".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pcgen_frng1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcgen_frng1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcgen_frng2(mut self, val: i32) -> Self {
        self.params.insert(
            "pcgen_frng2".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pcgen_frng2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcgen_frng2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_isamps(mut self, val: i32) -> Self {
        self.params.insert(
            "ss_isamps".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ss_isamps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_isamps".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_fsamps(mut self, val: i32) -> Self {
        self.params.insert(
            "ss_fsamps".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ss_fsamps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_fsamps".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_ba_wc(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "ba_wc".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_ba_wc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ba_wc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ba_wk(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "ba_wk".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_ba_wk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ba_wk".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngw1(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mf_rngw1".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mf_rngw1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngw1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_colw1(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mf_colw1".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mf_colw1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_colw1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngw2(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mf_rngw2".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mf_rngw2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngw2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_colw2(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mf_colw2".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mf_colw2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_colw2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngw3(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mf_rngw3".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mf_rngw3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngw3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_colw3(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mf_colw3".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mf_colw3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_colw3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngw4(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mf_rngw4".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mf_rngw4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngw4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_colw4(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mf_colw4".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mf_colw4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_colw4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngw5(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mf_rngw5".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mf_rngw5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngw5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_colw5(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mf_colw5".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mf_colw5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_colw5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_rngw6(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mf_rngw6".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mf_rngw6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_rngw6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_colw6(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mf_colw6".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mf_colw6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_colw6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngw1(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mn_rngw1".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mn_rngw1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngw1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_colw1(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mn_colw1".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mn_colw1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_colw1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngw2(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mn_rngw2".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mn_rngw2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngw2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_colw2(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mn_colw2".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mn_colw2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_colw2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngw3(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mn_rngw3".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mn_rngw3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngw3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_colw3(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mn_colw3".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mn_colw3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_colw3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_rngw4(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mn_rngw4".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mn_rngw4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_rngw4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_colw4(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "mn_colw4".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_mn_colw4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_colw4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_s_field(mut self, val: &str) -> Self {
        self.params.insert(
            "s_field".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_s_field_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_field".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_fieldm(mut self, val: &str) -> Self {
        self.params.insert(
            "s_fieldm".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_s_fieldm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_fieldm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_fieldkm(mut self, val: &str) -> Self {
        self.params.insert(
            "s_fieldkm".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_s_fieldkm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_fieldkm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_huemc(mut self, val: &str) -> Self {
        self.params.insert(
            "s_huemc".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_s_huemc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_huemc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_huemh(mut self, val: &str) -> Self {
        self.params.insert(
            "s_huemh".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_s_huemh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_huemh".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_huems(mut self, val: &str) -> Self {
        self.params.insert(
            "s_huems".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_s_huems_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_huems".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_albm(mut self, val: &str) -> Self {
        self.params.insert(
            "s_albm".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_s_albm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_albm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_shadhmc(mut self, val: &str) -> Self {
        self.params.insert(
            "s_shadhmc".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_s_shadhmc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_shadhmc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_shadhmh(mut self, val: &str) -> Self {
        self.params.insert(
            "s_shadhmh".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_s_shadhmh_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_shadhmh".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_shadhms(mut self, val: &str) -> Self {
        self.params.insert(
            "s_shadhms".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_s_shadhms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_shadhms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_shadkm(mut self, val: &str) -> Self {
        self.params.insert(
            "s_shadkm".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_s_shadkm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_shadkm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_phasemc(mut self, val: &str) -> Self {
        self.params.insert(
            "s_phasemc".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_s_phasemc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_phasemc".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_phasemr(mut self, val: &str) -> Self {
        self.params.insert(
            "s_phasemr".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_s_phasemr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_phasemr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_phasemg(mut self, val: &str) -> Self {
        self.params.insert(
            "s_phasemg".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_s_phasemg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_phasemg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_phasemb(mut self, val: &str) -> Self {
        self.params.insert(
            "s_phasemb".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_s_phasemb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_phasemb".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_tfield(mut self, val: &str) -> Self {
        self.params.insert(
            "b_tfield".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_b_tfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_tfield".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_tfieldm(mut self, val: &str) -> Self {
        self.params.insert(
            "b_tfieldm".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_b_tfieldm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_tfieldm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_ampm(mut self, val: &str) -> Self {
        self.params.insert(
            "b_ampm".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_b_ampm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_ampm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_dfield(mut self, val: &str) -> Self {
        self.params.insert(
            "b_dfield".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_b_dfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_dfield".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_dfieldm(mut self, val: &str) -> Self {
        self.params.insert(
            "b_dfieldm".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_b_dfieldm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_dfieldm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_kdensm(mut self, val: &str) -> Self {
        self.params.insert(
            "b_kdensm".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_b_kdensm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_kdensm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_blendm(mut self, val: &str) -> Self {
        self.params.insert(
            "b_blendm".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_b_blendm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_blendm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_model(mut self, val: &str) -> Self {
        self.params.insert(
            "b_model".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_b_model_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_model".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bph_tempm(mut self, val: &str) -> Self {
        self.params.insert(
            "bph_tempm".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bph_tempm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bph_tempm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bph_kcoolm(mut self, val: &str) -> Self {
        self.params.insert(
            "bph_kcoolm".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bph_kcoolm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bph_kcoolm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bph_bshiftm(mut self, val: &str) -> Self {
        self.params.insert(
            "bph_bshiftm".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bph_bshiftm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bph_bshiftm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_pcsmoke(mut self, val: &str) -> Self {
        self.params.insert(
            "ss_pcsmoke".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ss_pcsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_pcsmoke".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_pcfire(mut self, val: &str) -> Self {
        self.params.insert(
            "ss_pcfire".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ss_pcfire_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_pcfire".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcgen_cam(mut self, val: &str) -> Self {
        self.params.insert(
            "pcgen_cam".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pcgen_cam_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcgen_cam".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcgen_mode(mut self, val: &str) -> Self {
        self.params.insert(
            "pcgen_mode".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pcgen_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcgen_mode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_shtype(mut self, val: &str) -> Self {
        self.params.insert(
            "ss_shtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ss_shtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_shtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_label1(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_label1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_label1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_comment1(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_comment1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_comment1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_comment1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_field1(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_field1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_field1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_field1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_label2(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_label2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_label2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_comment2(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_comment2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_comment2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_comment2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_field2(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_field2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_field2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_field2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_label3(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_label3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_label3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_comment3(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_comment3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_comment3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_comment3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_field3(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_field3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_field3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_field3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_label4(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_label4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_label4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_comment4(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_comment4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_comment4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_comment4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_field4(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_field4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_field4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_field4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_label5(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_label5".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_label5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_label5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_comment5(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_comment5".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_comment5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_comment5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_field5(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_field5".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_field5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_field5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_label6(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_label6".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_label6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_label6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_comment6(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_comment6".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_comment6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_comment6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_field6(mut self, val: &str) -> Self {
        self.params.insert(
            "mf_field6".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mf_field6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_field6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_label1(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_label1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_label1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_comment1(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_comment1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_comment1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_comment1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_field1(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_field1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_field1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_field1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_basis1(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_basis1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_basis1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_basis1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_exponm1(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_exponm1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_exponm1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_exponm1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_flampm1(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_flampm1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_flampm1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_flampm1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_type1(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_type1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_type1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_type1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_octm1(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_octm1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_octm1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_octm1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_lacm1(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_lacm1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_lacm1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_lacm1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_gainm1(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_gainm1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_gainm1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_gainm1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_dispm1(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_dispm1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_dispm1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_dispm1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_ampm1(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_ampm1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_ampm1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_ampm1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_label2(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_label2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_label2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_comment2(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_comment2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_comment2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_comment2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_field2(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_field2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_field2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_field2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_basis2(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_basis2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_basis2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_basis2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_exponm2(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_exponm2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_exponm2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_exponm2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_flampm2(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_flampm2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_flampm2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_flampm2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_type2(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_type2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_type2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_type2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_octm2(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_octm2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_octm2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_octm2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_lacm2(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_lacm2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_lacm2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_lacm2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_gainm2(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_gainm2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_gainm2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_gainm2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_dispm2(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_dispm2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_dispm2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_dispm2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_ampm2(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_ampm2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_ampm2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_ampm2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_label3(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_label3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_label3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_comment3(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_comment3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_comment3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_comment3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_field3(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_field3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_field3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_field3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_basis3(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_basis3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_basis3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_basis3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_exponm3(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_exponm3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_exponm3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_exponm3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_flampm3(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_flampm3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_flampm3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_flampm3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_type3(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_type3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_type3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_type3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_octm3(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_octm3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_octm3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_octm3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_lacm3(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_lacm3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_lacm3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_lacm3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_gainm3(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_gainm3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_gainm3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_gainm3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_dispm3(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_dispm3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_dispm3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_dispm3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_ampm3(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_ampm3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_ampm3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_ampm3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_label4(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_label4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_label4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_comment4(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_comment4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_comment4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_comment4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_field4(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_field4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_field4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_field4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_basis4(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_basis4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_basis4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_basis4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_exponm4(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_exponm4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_exponm4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_exponm4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_flampm4(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_flampm4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_flampm4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_flampm4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_type4(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_type4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_type4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_type4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_octm4(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_octm4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_octm4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_octm4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_lacm4(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_lacm4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_lacm4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_lacm4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_gainm4(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_gainm4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_gainm4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_gainm4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_dispm4(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_dispm4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_dispm4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_dispm4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_ampm4(mut self, val: &str) -> Self {
        self.params.insert(
            "mn_ampm4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mn_ampm4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_ampm4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_label1(mut self, val: &str) -> Self {
        self.params.insert(
            "uf_label1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uf_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_label1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_comment1(mut self, val: &str) -> Self {
        self.params.insert(
            "uf_comment1".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uf_comment1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_comment1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_label2(mut self, val: &str) -> Self {
        self.params.insert(
            "uf_label2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uf_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_label2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_comment2(mut self, val: &str) -> Self {
        self.params.insert(
            "uf_comment2".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uf_comment2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_comment2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_label3(mut self, val: &str) -> Self {
        self.params.insert(
            "uf_label3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uf_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_label3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_comment3(mut self, val: &str) -> Self {
        self.params.insert(
            "uf_comment3".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uf_comment3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_comment3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_label4(mut self, val: &str) -> Self {
        self.params.insert(
            "uf_label4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uf_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_label4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_comment4(mut self, val: &str) -> Self {
        self.params.insert(
            "uf_comment4".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uf_comment4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_comment4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_label5(mut self, val: &str) -> Self {
        self.params.insert(
            "uf_label5".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uf_label5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_label5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_comment5(mut self, val: &str) -> Self {
        self.params.insert(
            "uf_comment5".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uf_comment5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_comment5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_label6(mut self, val: &str) -> Self {
        self.params.insert(
            "uf_label6".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uf_label6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_label6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_comment6(mut self, val: &str) -> Self {
        self.params.insert(
            "uf_comment6".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uf_comment6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_comment6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_s_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "s_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_s_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_usecd(mut self, val: bool) -> Self {
        self.params.insert(
            "s_usecd".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_s_usecd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_usecd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_ccenable(mut self, val: bool) -> Self {
        self.params.insert(
            "s_ccenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_s_ccenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s_ccenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "b_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_b_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_usecd(mut self, val: bool) -> Self {
        self.params.insert(
            "b_usecd".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_b_usecd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_usecd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bph_tksoft(mut self, val: bool) -> Self {
        self.params.insert(
            "bph_tksoft".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bph_tksoft_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bph_tksoft".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_b_ccenable(mut self, val: bool) -> Self {
        self.params.insert(
            "b_ccenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_b_ccenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "b_ccenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "ss_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ss_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcgen_tfrng(mut self, val: bool) -> Self {
        self.params.insert(
            "pcgen_tfrng".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pcgen_tfrng_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcgen_tfrng".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ss_ccenable(mut self, val: bool) -> Self {
        self.params.insert(
            "ss_ccenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ss_ccenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ss_ccenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_enable1(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_enable1".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_enable1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_enable1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_tsharp1(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_tsharp1".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_tsharp1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_tsharp1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_tsoft1(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_tsoft1".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_tsoft1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_tsoft1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_trng1(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_trng1".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_trng1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_trng1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_enable2(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_enable2".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_enable2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_enable2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_tsharp2(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_tsharp2".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_tsharp2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_tsharp2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_tsoft2(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_tsoft2".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_tsoft2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_tsoft2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_trng2(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_trng2".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_trng2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_trng2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_enable3(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_enable3".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_enable3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_enable3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_tsharp3(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_tsharp3".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_tsharp3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_tsharp3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_tsoft3(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_tsoft3".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_tsoft3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_tsoft3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_trng3(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_trng3".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_trng3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_trng3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_enable4(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_enable4".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_enable4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_enable4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_tsharp4(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_tsharp4".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_tsharp4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_tsharp4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_tsoft4(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_tsoft4".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_tsoft4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_tsoft4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_trng4(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_trng4".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_trng4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_trng4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_enable5(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_enable5".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_enable5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_enable5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_tsharp5(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_tsharp5".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_tsharp5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_tsharp5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_tsoft5(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_tsoft5".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_tsoft5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_tsoft5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_trng5(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_trng5".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_trng5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_trng5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_enable6(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_enable6".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_enable6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_enable6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_tsharp6(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_tsharp6".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_tsharp6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_tsharp6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_tsoft6(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_tsoft6".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_tsoft6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_tsoft6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mf_trng6(mut self, val: bool) -> Self {
        self.params.insert(
            "mf_trng6".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mf_trng6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mf_trng6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_enable1(mut self, val: bool) -> Self {
        self.params.insert(
            "mn_enable1".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mn_enable1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_enable1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_invert1(mut self, val: bool) -> Self {
        self.params.insert(
            "mn_invert1".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mn_invert1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_invert1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_is4d1(mut self, val: bool) -> Self {
        self.params.insert(
            "mn_is4d1".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mn_is4d1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_is4d1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_enable2(mut self, val: bool) -> Self {
        self.params.insert(
            "mn_enable2".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mn_enable2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_enable2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_invert2(mut self, val: bool) -> Self {
        self.params.insert(
            "mn_invert2".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mn_invert2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_invert2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_is4d2(mut self, val: bool) -> Self {
        self.params.insert(
            "mn_is4d2".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mn_is4d2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_is4d2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_enable3(mut self, val: bool) -> Self {
        self.params.insert(
            "mn_enable3".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mn_enable3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_enable3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_invert3(mut self, val: bool) -> Self {
        self.params.insert(
            "mn_invert3".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mn_invert3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_invert3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_is4d3(mut self, val: bool) -> Self {
        self.params.insert(
            "mn_is4d3".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mn_is4d3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_is4d3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_enable4(mut self, val: bool) -> Self {
        self.params.insert(
            "mn_enable4".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mn_enable4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_enable4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_invert4(mut self, val: bool) -> Self {
        self.params.insert(
            "mn_invert4".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mn_invert4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_invert4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mn_is4d5(mut self, val: bool) -> Self {
        self.params.insert(
            "mn_is4d5".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mn_is4d5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mn_is4d5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dr_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "dr_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dr_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dr_enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_enable1(mut self, val: bool) -> Self {
        self.params.insert(
            "uf_enable1".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uf_enable1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_enable1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_enable2(mut self, val: bool) -> Self {
        self.params.insert(
            "uf_enable2".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uf_enable2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_enable2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_enable3(mut self, val: bool) -> Self {
        self.params.insert(
            "uf_enable3".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uf_enable3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_enable3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_enable4(mut self, val: bool) -> Self {
        self.params.insert(
            "uf_enable4".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uf_enable4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_enable4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_enable5(mut self, val: bool) -> Self {
        self.params.insert(
            "uf_enable5".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uf_enable5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_enable5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uf_enable6(mut self, val: bool) -> Self {
        self.params.insert(
            "uf_enable6".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uf_enable6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uf_enable6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ShopPyro {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "pyro"
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
