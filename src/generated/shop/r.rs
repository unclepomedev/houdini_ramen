#[derive(Debug, Clone)]
pub struct ShopRandomwalkEval {
    pub base: crate::core::types::NodeBase,
}

impl ShopRandomwalkEval {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_pdf(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pdf".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pdf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_refl(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "refl".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refl_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "refl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eval(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "eval".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_eval_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "eval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_bounces(mut self, val: i32) -> Self {
        self.base.params.insert(
            "bounces".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bounces_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bounces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mybounces(mut self, val: i32) -> Self {
        self.base.params.insert(
            "mybounces".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mybounces_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mybounces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRandomwalkEval {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "randomwalk_eval"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRandomwalkSample {
    pub base: crate::core::types::NodeBase,
}

impl ShopRandomwalkSample {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_sx(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("sx".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_sx_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sy(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("sy".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_sy_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_g(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("g".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_g_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "g".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eta(mut self, val: f32) -> Self {
        self.base.params.insert(
            "eta".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eta_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "eta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdf(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pdf".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pdf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssg(mut self, val: f32) -> Self {
        self.base.params.insert(
            "ssg".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ssg_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ssg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sseta(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sseta".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sseta_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sseta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_u(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("u".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_u_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "u".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_absrp(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "absrp".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_absrp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "absrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scatr(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "scatr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scatr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scatr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "refl".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refl_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "refl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssabsrp(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "ssabsrp".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ssabsrp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ssabsrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssscatr(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "ssscatr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ssscatr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ssscatr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_bounces(mut self, val: i32) -> Self {
        self.base.params.insert(
            "bounces".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bounces_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bounces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bouncetype(mut self, val: i32) -> Self {
        self.base.params.insert(
            "bouncetype".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bouncetype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bouncetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssstype(mut self, val: i32) -> Self {
        self.base.params.insert(
            "ssstype".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ssstype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ssstype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mybounces(mut self, val: i32) -> Self {
        self.base.params.insert(
            "mybounces".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mybounces_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mybounces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRandomwalkSample {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "randomwalk_sample"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiBumpy {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiBumpy {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_amplitude(mut self, val: f32) -> Self {
        self.base.params.insert(
            "amplitude".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amplitude_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitude".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_texturename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "texturename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_texturename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "texturename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiBumpy {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_bumpy"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiCarpet {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiCarpet {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_kd(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Kd".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_kd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Kd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ka(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ka".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ka_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ka".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scuff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scuff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scuff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scuff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nap(mut self, val: f32) -> Self {
        self.base.params.insert(
            "nap".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_nap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "nap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiCarpet {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_carpet"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiCloth {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiCloth {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_freq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "freq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_freq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "freq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_depth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiCloth {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_cloth"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiCmarble {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiCmarble {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_ks(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ks".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ks_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kd(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Kd".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_kd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Kd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ka(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ka".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ka_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ka".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roughness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roughness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_veining(mut self, val: f32) -> Self {
        self.base.params.insert(
            "veining".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_veining_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "veining".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_specularcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_specularcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiCmarble {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_cmarble"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiConstant {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiConstant {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiConstant {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_constant"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiDelayedreadarchive {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiDelayedreadarchive {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- String parameters ---
    pub fn with_delayedreadarchive(mut self, val: &str) -> Self {
        self.base.params.insert(
            "DelayedReadArchive".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_delayedreadarchive_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "DelayedReadArchive".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiDelayedreadarchive {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_delayedreadarchive"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiDepthcue {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiDepthcue {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_mindistance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "mindistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mindistance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mindistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxdistance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxdistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxdistance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_background(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "background".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_background_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "background".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiDepthcue {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_depthcue"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiDiaknurl {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiDiaknurl {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_freq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "freq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_freq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "freq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_depth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_width(mut self, val: f32) -> Self {
        self.base.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_width_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_radius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zmin(mut self, val: f32) -> Self {
        self.base.params.insert(
            "zmin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zmin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "zmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zmax(mut self, val: f32) -> Self {
        self.base.params.insert(
            "zmax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zmax_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "zmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dampzone(mut self, val: f32) -> Self {
        self.base.params.insert(
            "dampzone".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dampzone_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dampzone".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_maporigin(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "maporigin".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_maporigin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maporigin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xaxis(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "xaxis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xaxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yaxis(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "yaxis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_yaxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "yaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zaxis(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "zaxis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_zaxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "zaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiDiaknurl {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_diaknurl"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiDistantlight {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiDistantlight {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_intensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_intensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_lightcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_from(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "from".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_from_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "from".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_to(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "to".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_to_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "to".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiDistantlight {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_distantlight"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiDynamicload {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiDynamicload {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- String parameters ---
    pub fn with_dynamicload(mut self, val: &str) -> Self {
        self.base.params.insert(
            "DynamicLoad".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dynamicload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "DynamicLoad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiDynamicload {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_dynamicload"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiFog {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiFog {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_distance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "distance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "distance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_background(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "background".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_background_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "background".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiFog {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_fog"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiGlass {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiGlass {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_ka(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ka".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ka_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ka".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kd(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Kd".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_kd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Kd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ks(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ks".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ks_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kr(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Kr".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_kr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Kr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roughness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roughness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_specularcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_specularcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_envname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "envname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_envname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "envname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiGlass {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_glass"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiGlassbal {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiGlassbal {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_ka(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ka".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ka_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ka".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ks(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ks".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ks_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roughness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roughness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kd(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Kd".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_kd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Kd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eta(mut self, val: f32) -> Self {
        self.base.params.insert(
            "eta".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eta_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "eta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_specularcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_specularcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_envname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "envname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_envname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "envname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiGlassbal {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_glassbal"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiLightlight {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiLightlight {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_intensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_intensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_lightcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_from(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "from".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_from_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "from".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiLightlight {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_lightlight"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiMatte {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiMatte {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_ka(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ka".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ka_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ka".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kd(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Kd".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_kd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Kd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiMatte {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_matte"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiMetal {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiMetal {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_ka(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ka".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ka_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ka".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ks(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ks".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ks_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roughness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roughness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiMetal {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_metal"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiPaintplastic {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiPaintplastic {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_ks(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ks".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ks_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kd(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Kd".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_kd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Kd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ka(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ka".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ka_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ka".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roughness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roughness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_specularcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_specularcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_texturename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "texturename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_texturename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "texturename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiPaintplastic {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_paintplastic"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiPlastic {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiPlastic {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_ks(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ks".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ks_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kd(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Kd".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_kd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Kd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ka(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ka".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ka_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ka".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roughness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roughness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_specularcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_specularcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiPlastic {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_plastic"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiPointnofallofflight {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiPointnofallofflight {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_intensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_intensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_lightcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_from(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "from".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_from_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "from".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiPointnofallofflight {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_pointnofallofflight"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiRmarble {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiRmarble {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_ks(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ks".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ks_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kd(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Kd".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_kd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Kd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ka(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ka".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ka_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ka".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roughness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roughness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_veining(mut self, val: f32) -> Self {
        self.base.params.insert(
            "veining".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_veining_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "veining".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_specularcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_specularcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiRmarble {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_rmarble"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiRsmetal {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiRsmetal {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_ka(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ka".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ka_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ka".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ks(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ks".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ks_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kr(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Kr".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_kr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Kr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roughness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roughness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiRsmetal {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_rsmetal"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiRunprogram {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiRunprogram {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- String parameters ---
    pub fn with_runprogram(mut self, val: &str) -> Self {
        self.base.params.insert(
            "RunProgram".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_runprogram_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "RunProgram".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiRunprogram {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_runprogram"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiShadowdistantlight {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiShadowdistantlight {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_intensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_intensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_samples(mut self, val: f32) -> Self {
        self.base.params.insert(
            "samples".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_samples_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_width(mut self, val: f32) -> Self {
        self.base.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_width_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_lightcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_from(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "from".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_from_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "from".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_to(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "to".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_to_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "to".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_shadowname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shadowname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shadowname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadowname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiShadowdistantlight {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_shadowdistantlight"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiShadowpointlight {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiShadowpointlight {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_intensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_intensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_width(mut self, val: f32) -> Self {
        self.base.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_width_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_samples(mut self, val: f32) -> Self {
        self.base.params.insert(
            "samples".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_samples_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_lightcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_from(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "from".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_from_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "from".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sfpx(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sfpx".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sfpx_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sfpx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sfnx(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sfnx".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sfnx_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sfnx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sfpy(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sfpy".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sfpy_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sfpy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sfny(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sfny".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sfny_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sfny".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sfpz(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sfpz".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sfpz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sfpz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sfnz(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sfnz".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sfnz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sfnz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiShadowpointlight {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_shadowpointlight"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiShadowspotlight {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiShadowspotlight {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_intensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_intensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coneangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coneangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coneangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coneangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_conedeltaangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "conedeltaangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_conedeltaangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "conedeltaangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beamdistribution(mut self, val: f32) -> Self {
        self.base.params.insert(
            "beamdistribution".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_beamdistribution_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "beamdistribution".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_samples(mut self, val: f32) -> Self {
        self.base.params.insert(
            "samples".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_samples_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_width(mut self, val: f32) -> Self {
        self.base.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_width_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_lightcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_from(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "from".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_from_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "from".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_to(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "to".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_to_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "to".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_shadowname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shadowname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shadowname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadowname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiShadowspotlight {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_shadowspotlight"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiShinymetal {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiShinymetal {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_ka(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ka".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ka_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ka".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ks(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ks".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ks_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kr(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Kr".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_kr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Kr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roughness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roughness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_texturename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "texturename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_texturename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "texturename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiShinymetal {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_shinymetal"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiSinknurl {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiSinknurl {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_freq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "freq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_freq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "freq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_depth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zmin(mut self, val: f32) -> Self {
        self.base.params.insert(
            "zmin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zmin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "zmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zmax(mut self, val: f32) -> Self {
        self.base.params.insert(
            "zmax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zmax_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "zmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dampzone(mut self, val: f32) -> Self {
        self.base.params.insert(
            "dampzone".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dampzone_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dampzone".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_maporigin(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "maporigin".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_maporigin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maporigin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xaxis(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "xaxis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xaxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yaxis(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "yaxis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_yaxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "yaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zaxis(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "zaxis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_zaxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "zaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiSinknurl {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_sinknurl"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiSpatter {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiSpatter {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_ka(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ka".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ka_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ka".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kd(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Kd".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_kd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Kd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ks(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ks".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ks_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roughness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roughness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specksize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "specksize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_specksize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "specksize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sizes(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sizes".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sizes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sizes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_specularcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_specularcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basecolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "basecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_basecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "basecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spattercolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "spattercolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spattercolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spattercolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiSpatter {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_spatter"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiSpotlight {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiSpotlight {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_intensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_intensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coneangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "coneangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coneangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coneangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_conedeltaangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "conedeltaangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_conedeltaangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "conedeltaangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beamdistribution(mut self, val: f32) -> Self {
        self.base.params.insert(
            "beamdistribution".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_beamdistribution_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "beamdistribution".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_lightcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_from(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "from".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_from_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "from".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_to(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "to".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_to_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "to".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiSpotlight {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_spotlight"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiStippled {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiStippled {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_ks(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ks".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ks_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kd(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Kd".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_kd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Kd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ka(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ka".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ka_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ka".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roughness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roughness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_grainsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "grainsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_grainsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "grainsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stippling(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stippling".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stippling_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stippling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_specularcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_specularcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiStippled {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_stippled"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiStone {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiStone {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_ks(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ks".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ks_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kd(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Kd".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_kd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Kd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ka(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Ka".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ka_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Ka".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roughness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roughness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roughness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nshades(mut self, val: f32) -> Self {
        self.base.params.insert(
            "nshades".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_nshades_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "nshades".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exponent(mut self, val: f32) -> Self {
        self.base.params.insert(
            "exponent".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_exponent_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exponent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_specularcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_specularcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "specularcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_graincolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "graincolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_graincolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "graincolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiStone {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_stone"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRiThreads {
    pub base: crate::core::types::NodeBase,
}

impl ShopRiThreads {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_freq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "freq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_freq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "freq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_depth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "depth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_phase(mut self, val: f32) -> Self {
        self.base.params.insert(
            "phase".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_phase_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "phase".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zmin(mut self, val: f32) -> Self {
        self.base.params.insert(
            "zmin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zmin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "zmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zmax(mut self, val: f32) -> Self {
        self.base.params.insert(
            "zmax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zmax_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "zmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dampzone(mut self, val: f32) -> Self {
        self.base.params.insert(
            "dampzone".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dampzone_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dampzone".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_maporigin(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "maporigin".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_maporigin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maporigin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xaxis(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "xaxis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xaxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yaxis(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "yaxis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_yaxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "yaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_zaxis(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "zaxis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_zaxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "zaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRiThreads {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ri_threads"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRisnet {
    pub base: crate::core::types::NodeBase,
}

impl ShopRisnet {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRisnet {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "risnet"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRopnet {
    pub base: crate::core::types::NodeBase,
}

impl ShopRopnet {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_execute(mut self) -> Self {
        self.base.params.insert(
            "execute".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.base.params.insert(
            "renderdialog".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRopnet {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ropnet"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRslVopdisplace {
    pub base: crate::core::types::NodeBase,
}

impl ShopRslVopdisplace {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.base.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRslVopdisplace {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_vopdisplace"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRslVopdisplacetype {
    pub base: crate::core::types::NodeBase,
}

impl ShopRslVopdisplacetype {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- String parameters ---
    pub fn with_englishname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_englishname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_tabmenuflag(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tabmenuflag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRslVopdisplacetype {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_vopdisplacetype"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRslVopimager {
    pub base: crate::core::types::NodeBase,
}

impl ShopRslVopimager {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.base.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRslVopimager {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_vopimager"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRslVoplight {
    pub base: crate::core::types::NodeBase,
}

impl ShopRslVoplight {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.base.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRslVoplight {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_voplight"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRslVoplighttype {
    pub base: crate::core::types::NodeBase,
}

impl ShopRslVoplighttype {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- String parameters ---
    pub fn with_englishname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_englishname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_tabmenuflag(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tabmenuflag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRslVoplighttype {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_voplighttype"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRslVopmaterial {
    pub base: crate::core::types::NodeBase,
}

impl ShopRslVopmaterial {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.base.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRslVopmaterial {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_vopmaterial"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRslVopshaderclass {
    pub base: crate::core::types::NodeBase,
}

impl ShopRslVopshaderclass {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.base.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRslVopshaderclass {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_vopshaderclass"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRslVopstruct {
    pub base: crate::core::types::NodeBase,
}

impl ShopRslVopstruct {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.base.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRslVopstruct {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_vopstruct"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRslVopsurface {
    pub base: crate::core::types::NodeBase,
}

impl ShopRslVopsurface {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.base.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_ogl_rough(mut self, val: f32) -> Self {
        self.base.params.insert(
            "ogl_rough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_rough_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ogl_rough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_alpha(mut self, val: f32) -> Self {
        self.base.params.insert(
            "ogl_alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_alpha_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ogl_alpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_ogl_amb(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "ogl_amb".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_amb_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ogl_amb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_diff(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "ogl_diff".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_diff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ogl_diff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_spec(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "ogl_spec".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_spec_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ogl_spec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_emit(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "ogl_emit".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_emit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ogl_emit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_ogl_tex_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("ogl_tex{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("ogl_tex{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_min_filter_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("ogl_tex_min_filter{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_min_filter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("ogl_tex_min_filter{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_mag_filter_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("ogl_tex_mag_filter{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_mag_filter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("ogl_tex_mag_filter{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_ogl_light(mut self, val: bool) -> Self {
        self.base.params.insert(
            "ogl_light".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_light_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ogl_light".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_cutout(mut self, val: bool) -> Self {
        self.base.params.insert(
            "ogl_cutout".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_cutout_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ogl_cutout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_clamping_mode_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("ogl_clamping_mode{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_clamping_mode_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("ogl_clamping_mode{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRslVopsurface {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_vopsurface"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRslVopsurfacetype {
    pub base: crate::core::types::NodeBase,
}

impl ShopRslVopsurfacetype {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- String parameters ---
    pub fn with_englishname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_englishname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_tabmenuflag(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tabmenuflag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRslVopsurfacetype {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_vopsurfacetype"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRslVopvolume {
    pub base: crate::core::types::NodeBase,
}

impl ShopRslVopvolume {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.base.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRslVopvolume {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_vopvolume"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ShopRslVopvolumetype {
    pub base: crate::core::types::NodeBase,
}

impl ShopRslVopvolumetype {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- String parameters ---
    pub fn with_englishname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_englishname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_tabmenuflag(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tabmenuflag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopRslVopvolumetype {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_vopvolumetype"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}
