#[derive(Debug, Clone)]
pub struct ShopHairEval {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopHairEval {
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
    pub fn with_lobe_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "lobe_shift".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lobe_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lobe_shift".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lobe_width_lon(mut self, val: f32) -> Self {
        self.params.insert(
            "lobe_width_lon".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lobe_width_lon_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lobe_width_lon".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lobe_width_azi(mut self, val: f32) -> Self {
        self.params.insert(
            "lobe_width_azi".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lobe_width_azi_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lobe_width_azi".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glint_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "glint_shift".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glint_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glint_shift".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glint_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "glint_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glint_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glint_intensity".to_string(),
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
    pub fn with_v(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("v".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "v".to_string(),
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
    pub fn with_n(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("N".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_n_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "N".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tip(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tip".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tip".to_string(),
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
    pub fn with_reverse(mut self, val: i32) -> Self {
        self.params.insert(
            "reverse".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_reverse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reverse".to_string(),
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

impl crate::core::types::HoudiniNode for ShopHairEval {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "hair_eval"
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
pub struct ShopHairSample {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopHairSample {
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
    pub fn with_lobe_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "lobe_shift".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lobe_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lobe_shift".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lobe_width_lon(mut self, val: f32) -> Self {
        self.params.insert(
            "lobe_width_lon".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lobe_width_lon_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lobe_width_lon".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lobe_width_azi(mut self, val: f32) -> Self {
        self.params.insert(
            "lobe_width_azi".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lobe_width_azi_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lobe_width_azi".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glint_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "glint_shift".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glint_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glint_shift".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glint_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "glint_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glint_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glint_intensity".to_string(),
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
    pub fn with_v(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("v".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "v".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_n(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("N".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_n_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "N".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tip(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tip".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_flags(mut self, val: i32) -> Self {
        self.params.insert(
            "flags".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_flags_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flags".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

impl crate::core::types::HoudiniNode for ShopHairSample {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "hair_sample"
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
pub struct ShopHairshader {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopHairshader {
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
    pub fn with_kd(mut self, val: f32) -> Self {
        self.params
            .insert("Kd".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_kd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Kd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colormapwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "colorMapWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colormapwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorMapWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colormapintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "colorMapIntensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colormapintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorMapIntensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tipcolormapwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "tipColorMapWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tipcolormapwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipColorMapWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tipcolormapintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "tipColorMapIntensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tipcolormapintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipColorMapIntensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trt_size(mut self, val: f32) -> Self {
        self.params.insert(
            "trt_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trt_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trt_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "trt_shift".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trt_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_shift".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_int(mut self, val: f32) -> Self {
        self.params.insert(
            "spec_int".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spec_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_int".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intensitymapwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "intensityMapWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_intensitymapwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "intensityMapWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rootmapwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "rootMapWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rootmapwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rootMapWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colormapintensity1(mut self, val: f32) -> Self {
        self.params.insert(
            "colorMapIntensity1".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_colormapintensity1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorMapIntensity1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tipspecmapwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "tipSpecMapWidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tipspecmapwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipSpecMapWidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tipspecmapintensity(mut self, val: f32) -> Self {
        self.params.insert(
            "tipSpecMapIntensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tipspecmapintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipSpecMapIntensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tt_int(mut self, val: f32) -> Self {
        self.params.insert(
            "tt_int".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tt_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_int".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tt_size(mut self, val: f32) -> Self {
        self.params.insert(
            "tt_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tt_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tt_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "tt_shift".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tt_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_shift".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_alpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_2_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("ogl_tex{}_2", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_tex_2_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex{}_2", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_min_filter_2_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("ogl_tex_min_filter{}_2", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_tex_min_filter_2_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_min_filter{}_2", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_mag_filter_2_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("ogl_tex_mag_filter{}_2", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_tex_mag_filter_2_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_mag_filter{}_2", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_clamping_mode_2_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("ogl_clamping_mode{}_2", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_clamping_mode_2_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_clamping_mode{}_2", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_rough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_rough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpscale(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_bumpscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_bumpscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumpscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_envscale(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_envscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_envscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_envscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_diff_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_hair_diff_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_spec_intensity(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_hair_spec_intensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_hair_spec_intensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_intensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_spec_shift(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_hair_spec_shift".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_hair_spec_shift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_shift".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "ogl_hair_alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ogl_hair_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_alpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_direct_samples(mut self, val: f32) -> Self {
        self.params.insert(
            "direct_samples".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_direct_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_level(mut self, val: f32) -> Self {
        self.params.insert(
            "level".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_level_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "level".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuselevel(mut self, val: f32) -> Self {
        self.params.insert(
            "diffuselevel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diffuselevel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuselevel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specularlevel(mut self, val: f32) -> Self {
        self.params.insert(
            "specularlevel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_specularlevel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specularlevel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumelevel(mut self, val: f32) -> Self {
        self.params.insert(
            "volumelevel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumelevel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumelevel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirect_samples(mut self, val: f32) -> Self {
        self.params.insert(
            "indirect_samples".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_indirect_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirect_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nlights(mut self, val: f32) -> Self {
        self.params.insert(
            "nlights".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_nlights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nlights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nddispersion(mut self, val: f32) -> Self {
        self.params.insert(
            "nddispersion".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_nddispersion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nddispersion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ndpriority(mut self, val: f32) -> Self {
        self.params.insert(
            "ndpriority".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ndpriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ndpriority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ndior(mut self, val: f32) -> Self {
        self.params.insert(
            "ndior".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ndior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ndior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_diffuserandhuerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "diffuseRandHueRange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_diffuserandhuerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandHueRange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuserandsaturationrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "diffuseRandSaturationRange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_diffuserandsaturationrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandSaturationRange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuserandintensityrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "diffuseRandIntensityRange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_diffuserandintensityrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandIntensityRange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specrandintensityrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "specRandIntensityRange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_specrandintensityrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specRandIntensityRange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_shinyrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "ogl_shinyrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ogl_shinyrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_shinyrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_hue_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_hue_range".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_hue_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_hue_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_sat_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_sat_range".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_sat_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_sat_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_int_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_int_range".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_int_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_int_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_spec_random_int_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "ogl_hair_spec_random_int_range".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_ogl_hair_spec_random_int_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_random_int_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_rootdiffusecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rootDiffuseColor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rootdiffusecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rootDiffuseColor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tipdiffusecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tipDiffuseColor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tipdiffusecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipDiffuseColor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_whitehaircolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "whitehaircolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_whitehaircolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "whitehaircolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guardhairtintcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guardhairtintcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guardhairtintcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guardhairtintcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rootspeccolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rootSpecColor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rootspeccolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rootSpecColor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tipspeccolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tipSpecColor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tipspeccolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipSpecColor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tt_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tt_clr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tt_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_clr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opac_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "opac_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_opac_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opac_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_amb(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_amb".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_amb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_amb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_emit(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_emit".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_emit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_emit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_diff(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_diff".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_diff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_diff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_spec(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_spec".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_spec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_envrotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_envrotate".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_envrotate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_envrotate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_diff_root_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_hair_diff_root_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_root_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_root_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_diff_tip_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_hair_diff_tip_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_tip_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_tip_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_spec_root_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_hair_spec_root_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_hair_spec_root_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_root_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_spec_tip_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ogl_hair_spec_tip_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ogl_hair_spec_tip_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_tip_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuse_global_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "diffuse_global_clr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_diffuse_global_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuse_global_clr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opacityexport(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "opacityExport".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_opacityexport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacityExport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_direct(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direct".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direct_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_direct_noshadow(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direct_noshadow".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direct_noshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct_noshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_direct_shadow(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direct_shadow".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direct_shadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct_shadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_direct_diffuse(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direct_diffuse".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direct_diffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct_diffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_direct_reflect(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direct_reflect".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direct_reflect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct_reflect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_direct_refract(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direct_refract".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direct_refract_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct_refract".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_direct_specular(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direct_specular".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direct_specular_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct_specular".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_direct_volume(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direct_volume".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direct_volume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct_volume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirect_reflect(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "indirect_reflect".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_indirect_reflect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirect_reflect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirect_refract(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "indirect_refract".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_indirect_refract_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirect_refract".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirect(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "indirect".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_indirect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ce(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ce".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_direct_emission(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direct_emission".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direct_emission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct_emission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_all_emission(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "all_emission".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_all_emission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "all_emission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_all(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "all".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_all_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "all".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirect_emission(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "indirect_emission".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_indirect_emission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirect_emission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirect_noshadow(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "indirect_noshadow".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_indirect_noshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirect_noshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirect_shadow(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "indirect_shadow".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_indirect_shadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirect_shadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_absorption(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "absorption".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_absorption_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "absorption".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_diffuserandhueseed(mut self, val: i32) -> Self {
        self.params.insert(
            "diffuseRandHueSeed".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_diffuserandhueseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandHueSeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuserandsaturationseed(mut self, val: i32) -> Self {
        self.params.insert(
            "diffuseRandSaturationSeed".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_diffuserandsaturationseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandSaturationSeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuserandintensityseed(mut self, val: i32) -> Self {
        self.params.insert(
            "diffuseRandIntensitySeed".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_diffuserandintensityseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandIntensitySeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specrandintensityseed(mut self, val: i32) -> Self {
        self.params.insert(
            "specRandIntensitySeed".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_specrandintensityseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specRandIntensitySeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_speclayer(mut self, val: i32) -> Self {
        self.params.insert(
            "ogl_speclayer".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ogl_speclayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_speclayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_opacitylayer(mut self, val: i32) -> Self {
        self.params.insert(
            "ogl_opacitylayer".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ogl_opacitylayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_opacitylayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumplayer(mut self, val: i32) -> Self {
        self.params.insert(
            "ogl_bumplayer".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ogl_bumplayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumplayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_normallayer(mut self, val: i32) -> Self {
        self.params.insert(
            "ogl_normallayer".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ogl_normallayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normallayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_whitehair(mut self, val: i32) -> Self {
        self.params.insert(
            "whitehair".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_whitehair_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "whitehair".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guardhair(mut self, val: i32) -> Self {
        self.params.insert(
            "guardhair".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_guardhair_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guardhair".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hairid(mut self, val: i32) -> Self {
        self.params.insert(
            "hairid".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hairid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hairid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_diffuseblendramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "diffuseBlendRamp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_diffuseblendramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseBlendRamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specblendramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "specBlendRamp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_specblendramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specBlendRamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuserandhueramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "diffuseRandHueRamp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_diffuserandhueramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandHueRamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuserandsaturationramp(
        mut self,
        val: Vec<crate::core::types::RampPoint>,
    ) -> Self {
        self.params.insert(
            "diffuseRandSaturationRamp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_diffuserandsaturationramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandSaturationRamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuserandintensityramp(
        mut self,
        val: Vec<crate::core::types::RampPoint>,
    ) -> Self {
        self.params.insert(
            "diffuseRandIntensityRamp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_diffuserandintensityramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandIntensityRamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specrandintensityramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "specRandIntensityRamp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_specrandintensityramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specRandIntensityRamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opacityramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "opacityramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_opacityramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacityramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_baserootcolormap(mut self, val: &str) -> Self {
        self.params.insert(
            "baseRootColorMap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_baserootcolormap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "baseRootColorMap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorrootmapfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "colorRootMapFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_colorrootmapfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorRootMapFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tipcolormap(mut self, val: &str) -> Self {
        self.params.insert(
            "tipColorMap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tipcolormap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipColorMap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tipcolormapfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "tipColorMapFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tipcolormapfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipColorMapFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_baseintensitymap(mut self, val: &str) -> Self {
        self.params.insert(
            "baseIntensityMap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_baseintensitymap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "baseIntensityMap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intensitymapfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "intensityMapFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_intensitymapfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "intensityMapFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rootspecmap(mut self, val: &str) -> Self {
        self.params.insert(
            "rootSpecMap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rootspecmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rootSpecMap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rootspecmapfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "rootSpecMapFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rootspecmapfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rootSpecMapFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basetipspecmap(mut self, val: &str) -> Self {
        self.params.insert(
            "baseTipSpecMap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_basetipspecmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "baseTipSpecMap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tipspecmapfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "tipSpecMapFilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tipspecmapfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipSpecMapFilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ogl_tex{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_min_filter_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_min_filter{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_min_filter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_min_filter{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_mag_filter_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_mag_filter{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_tex_mag_filter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_tex_mag_filter{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_specmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_specmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_specmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_specmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_opacitymap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_opacitymap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_opacitymap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_opacitymap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_bumpmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumpmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumptype(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_bumptype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_bumptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpbias(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_bumpbias".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumpbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_normalmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_normalmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_normalmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normalmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_normalbias(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_normalbias".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_normalbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_normalbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_envmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_envmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_envmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_envmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_envrotorder(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_envrotorder".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_envrotorder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_envrotorder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_diff_map(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_map".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_diff_map_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_map".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_diff_blend_ramp(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_blend_ramp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_diff_blend_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_blend_ramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_spec_map(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_map".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_spec_map_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_map".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_spec_blend_ramp(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_blend_ramp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_spec_blend_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_blend_ramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_alpha_blend_ramp(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_hair_alpha_blend_ramp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_alpha_blend_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_alpha_blend_ramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usepointcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "usePointColor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepointcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usePointColor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_userootdiffusemap(mut self, val: bool) -> Self {
        self.params.insert(
            "useRootDiffuseMap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userootdiffusemap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useRootDiffuseMap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tipdiffuseseparate(mut self, val: bool) -> Self {
        self.params.insert(
            "tipDiffuseSeparate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tipdiffuseseparate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipDiffuseSeparate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetipdiffusemap(mut self, val: bool) -> Self {
        self.params.insert(
            "useTipDiffuseMap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetipdiffusemap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useTipDiffuseMap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usewhitehairs(mut self, val: bool) -> Self {
        self.params.insert(
            "useWhiteHairs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usewhitehairs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useWhiteHairs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useguardhairs(mut self, val: bool) -> Self {
        self.params.insert(
            "useGuardHairs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useguardhairs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useGuardHairs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trt_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "trt_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_trt_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trt_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usespecintensitymap(mut self, val: bool) -> Self {
        self.params.insert(
            "useSpecIntensityMap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usespecintensitymap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useSpecIntensityMap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_userootspecmap(mut self, val: bool) -> Self {
        self.params.insert(
            "useRootSpecMap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userootspecmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useRootSpecMap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tipspecseparate(mut self, val: bool) -> Self {
        self.params.insert(
            "tipSpecSeparate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tipspecseparate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tipSpecSeparate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetipspecmap(mut self, val: bool) -> Self {
        self.params.insert(
            "useTipSpecMap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetipspecmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useTipSpecMap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tt_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "tt_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tt_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tt_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tinttransmissionwithprimary(mut self, val: bool) -> Self {
        self.params.insert(
            "tintTransmissionWithPrimary".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tinttransmissionwithprimary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tintTransmissionWithPrimary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuserandhue(mut self, val: bool) -> Self {
        self.params.insert(
            "diffuseRandHue".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_diffuserandhue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandHue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuserandsaturation(mut self, val: bool) -> Self {
        self.params.insert(
            "diffuseRandSaturation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_diffuserandsaturation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandSaturation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuserandintensity(mut self, val: bool) -> Self {
        self.params.insert(
            "diffuseRandIntensity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_diffuserandintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuseRandIntensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specrandintensity(mut self, val: bool) -> Self {
        self.params.insert(
            "specRandIntensity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_specrandintensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specRandIntensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_light(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_light".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_light_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_light".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_cutout(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_cutout".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_cutout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_cutout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_clamping_mode_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("ogl_clamping_mode{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_clamping_mode_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ogl_clamping_mode{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_adjustshiny(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_adjustshiny".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_adjustshiny_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_adjustshiny".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_bumpinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_bumpinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_bumpinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_bumpinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_use_diff_map(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_hair_use_diff_map".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_hair_use_diff_map_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_use_diff_map".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_use_spec_map(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_hair_use_spec_map".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_hair_use_spec_map_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_use_spec_map".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_hue(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_hue".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_hue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_hue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_sat(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_sat".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_sat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_sat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_int(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_int".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_hair_diff_random_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_diff_random_int".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_hair_spec_random_int(mut self, val: bool) -> Self {
        self.params.insert(
            "ogl_hair_spec_random_int".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ogl_hair_spec_random_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_hair_spec_random_int".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopHairshader {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "hairshader"
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
pub struct ShopHenyeygreensteinEval {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopHenyeygreensteinEval {
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
    pub fn with_v(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("v".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "v".to_string(),
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
    pub fn with_reverse(mut self, val: i32) -> Self {
        self.params.insert(
            "reverse".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_reverse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reverse".to_string(),
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

impl crate::core::types::HoudiniNode for ShopHenyeygreensteinEval {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "henyeygreenstein_eval"
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
pub struct ShopHenyeygreensteinSample {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopHenyeygreensteinSample {
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
    pub fn with_v(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("v".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "v".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_flags(mut self, val: i32) -> Self {
        self.params.insert(
            "flags".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_flags_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flags".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

impl crate::core::types::HoudiniNode for ShopHenyeygreensteinSample {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "henyeygreenstein_sample"
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
