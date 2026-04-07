#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopVAluminumType {
    Ward = 0,
    Ashikhmin = 1,
}

#[derive(Debug, Clone)]
pub struct ShopVAluminum {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVAluminum {
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
    pub fn with_urough(mut self, val: f32) -> Self {
        self.params.insert(
            "urough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_urough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "urough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vrough(mut self, val: f32) -> Self {
        self.params.insert(
            "vrough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vrough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vrough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_diff(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "diff".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_spec(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spec".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_type(mut self, val: ShopVAluminumType) -> Self {
        self.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
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
    pub fn with_tstyle(mut self, val: &str) -> Self {
        self.params.insert(
            "tstyle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tstyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tstyle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVAluminum {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_aluminum"
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
pub struct ShopVAmbient {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVAmbient {
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

    // --- Float3 parameters ---
    pub fn with_lightcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVAmbient {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_ambient"
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
pub struct ShopVArealight {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVArealight {
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
    pub fn with_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_edgewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "edgewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_edgewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "edgewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_edgerolloff(mut self, val: f32) -> Self {
        self.params.insert(
            "edgerolloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_edgerolloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "edgerolloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_envblurangle(mut self, val: f32) -> Self {
        self.params.insert(
            "envblurangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_envblurangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envblurangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_envscale(mut self, val: f32) -> Self {
        self.params.insert(
            "envscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_envscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_lightcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_envclipy(mut self, val: i32) -> Self {
        self.params.insert(
            "envclipy".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_envclipy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envclipy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_texmap(mut self, val: &str) -> Self {
        self.params.insert(
            "texmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_texmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "texmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_envmap(mut self, val: &str) -> Self {
        self.params.insert(
            "envmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_envmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_envnull(mut self, val: &str) -> Self {
        self.params.insert(
            "envnull".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_envnull_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envnull".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_doedge(mut self, val: bool) -> Self {
        self.params.insert(
            "doedge".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doedge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doedge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_singlesided(mut self, val: bool) -> Self {
        self.params.insert(
            "singlesided".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_singlesided_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "singlesided".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reverse(mut self, val: bool) -> Self {
        self.params.insert(
            "reverse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
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
    pub fn with_normalizearea(mut self, val: bool) -> Self {
        self.params.insert(
            "normalizearea".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normalizearea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalizearea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVArealight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_arealight"
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
pub struct ShopVAsadlens {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVAsadlens {
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
    pub fn with_zoom(mut self, val: f32) -> Self {
        self.params.insert(
            "zoom".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_zoom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zoom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_curvature(mut self, val: f32) -> Self {
        self.params.insert(
            "curvature".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_curvature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_focus(mut self, val: f32) -> Self {
        self.params.insert(
            "focus".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_focus_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "focus".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coc(mut self, val: f32) -> Self {
        self.params.insert(
            "coc".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_projection(mut self, val: &str) -> Self {
        self.params.insert(
            "projection".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_projection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVAsadlens {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_asadlens"
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
pub enum ShopVAsadlightDoatten {
    NoAttenuation = 0,
    HalfIntensityDistance = 1,
    PhysicallyAccurate = 2,
    Custom = 3,
}

#[derive(Debug, Clone)]
pub struct ShopVAsadlight {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVAsadlight {
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
    pub fn with_coneangle(mut self, val: f32) -> Self {
        self.params.insert(
            "coneangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coneangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coneangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_conedelta(mut self, val: f32) -> Self {
        self.params.insert(
            "conedelta".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_conedelta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "conedelta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_conerolloff(mut self, val: f32) -> Self {
        self.params.insert(
            "conerolloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_conerolloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "conerolloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attenstart(mut self, val: f32) -> Self {
        self.params.insert(
            "attenstart".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_attenstart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attenstart".to_string(),
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
    pub fn with_cnstatten(mut self, val: f32) -> Self {
        self.params.insert(
            "cnstatten".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cnstatten_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cnstatten".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_linatten(mut self, val: f32) -> Self {
        self.params.insert(
            "linatten".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_linatten_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "linatten".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quadatten(mut self, val: f32) -> Self {
        self.params.insert(
            "quadatten".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_quadatten_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quadatten".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rampstart(mut self, val: f32) -> Self {
        self.params.insert(
            "rampstart".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rampstart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rampstart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rampend(mut self, val: f32) -> Self {
        self.params.insert(
            "rampend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rampend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rampend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_envblurangle(mut self, val: f32) -> Self {
        self.params.insert(
            "envblurangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_envblurangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envblurangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_envscale(mut self, val: f32) -> Self {
        self.params.insert(
            "envscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_envscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflectx(mut self, val: f32) -> Self {
        self.params.insert(
            "reflectx".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflectx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflectx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflecty(mut self, val: f32) -> Self {
        self.params.insert(
            "reflecty".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflecty_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflecty".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_lightcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outside(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "outside".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_outside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_doatten(mut self, val: ShopVAsadlightDoatten) -> Self {
        self.params.insert(
            "doatten".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_doatten_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doatten".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_envclipy(mut self, val: i32) -> Self {
        self.params.insert(
            "envclipy".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_envclipy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envclipy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_atten_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "atten_ramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_atten_ramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "atten_ramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_slide(mut self, val: &str) -> Self {
        self.params.insert(
            "slide".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_slide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_envmap(mut self, val: &str) -> Self {
        self.params.insert(
            "envmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_envmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_envnull(mut self, val: &str) -> Self {
        self.params.insert(
            "envnull".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_envnull_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envnull".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffmap(mut self, val: &str) -> Self {
        self.params.insert(
            "diffmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_diffmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specmap(mut self, val: &str) -> Self {
        self.params.insert(
            "specmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_specmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_docone(mut self, val: bool) -> Self {
        self.params.insert(
            "docone".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docone_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docone".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharpspot(mut self, val: bool) -> Self {
        self.params.insert(
            "sharpspot".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sharpspot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharpspot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distant(mut self, val: bool) -> Self {
        self.params.insert(
            "distant".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_distant_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distant".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doattenramp(mut self, val: bool) -> Self {
        self.params.insert(
            "doattenramp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doattenramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doattenramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVAsadlight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_asadlight"
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
pub struct ShopVAttenlight {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVAttenlight {
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

    // --- Float3 parameters ---
    pub fn with_lightcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVAttenlight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_attenlight"
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
pub struct ShopVAttenspot {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVAttenspot {
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
    pub fn with_coneangle(mut self, val: f32) -> Self {
        self.params.insert(
            "coneangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coneangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coneangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_conedeltaangle(mut self, val: f32) -> Self {
        self.params.insert(
            "conedeltaangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_conedeltaangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "conedeltaangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_beamdistribution(mut self, val: f32) -> Self {
        self.params.insert(
            "beamdistribution".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_beamdistribution_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "beamdistribution".to_string(),
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

    // --- Float3 parameters ---
    pub fn with_lightcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVAttenspot {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_attenspot"
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
pub struct ShopVBlurshadow {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVBlurshadow {
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
    pub fn with_spread(mut self, val: f32) -> Self {
        self.params.insert(
            "spread".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spread_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spread".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quality(mut self, val: f32) -> Self {
        self.params.insert(
            "quality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_quality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowi(mut self, val: f32) -> Self {
        self.params.insert(
            "shadowI".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadowi_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowI".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fog_factor(mut self, val: f32) -> Self {
        self.params.insert(
            "fog_factor".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fog_factor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fog_factor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_shadowtype(mut self, val: &str) -> Self {
        self.params.insert(
            "shadowtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shadowtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVBlurshadow {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_blurshadow"
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
pub struct ShopVBurlap {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVBurlap {
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
    pub fn with_xsize(mut self, val: f32) -> Self {
        self.params.insert(
            "xsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ysize(mut self, val: f32) -> Self {
        self.params.insert(
            "ysize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ysize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ysize".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_forpoly(mut self, val: bool) -> Self {
        self.params.insert(
            "forpoly".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forpoly_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forpoly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVBurlap {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_burlap"
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
pub struct ShopVCartoon {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVCartoon {
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

    // --- Float3 parameters ---
    pub fn with_baseclr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "baseclr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_baseclr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "baseclr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVCartoon {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_cartoon"
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
pub struct ShopVChoppywater {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVChoppywater {
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
    pub fn with_height(mut self, val: f32) -> Self {
        self.params.insert(
            "height".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_height_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "height".to_string(),
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
    pub fn with_secondary(mut self, val: f32) -> Self {
        self.params.insert(
            "secondary".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_secondary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "secondary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_base(mut self, val: f32) -> Self {
        self.params.insert(
            "base".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "base".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_forpoly(mut self, val: bool) -> Self {
        self.params.insert(
            "forpoly".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forpoly_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forpoly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVChoppywater {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_choppywater"
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
pub struct ShopVClay {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVClay {
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

    // --- Float3 parameters ---
    pub fn with_amb(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "amb".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_amb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "diff".to_string(),
            crate::core::types::ParamValue::Float3(val),
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

impl crate::core::types::HoudiniNode for ShopVClay {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_clay"
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
pub struct ShopVCollada {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVCollada {
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
    pub fn with_shininess(mut self, val: f32) -> Self {
        self.params.insert(
            "shininess".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shininess_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shininess".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
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
    pub fn with_ca(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_lmodel(mut self, val: &str) -> Self {
        self.params.insert(
            "lmodel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map(mut self, val: &str) -> Self {
        self.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVCollada {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_collada"
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
pub struct ShopVConstant {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVConstant {
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
    pub fn with_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "clr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opac(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "opac".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_opac_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opac".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
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
}

impl crate::core::types::HoudiniNode for ShopVConstant {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_constant"
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
pub struct ShopVCorrugated {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVCorrugated {
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
    pub fn with_freq(mut self, val: f32) -> Self {
        self.params.insert(
            "freq".to_string(),
            crate::core::types::ParamValue::Float(val),
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
    pub fn with_power(mut self, val: f32) -> Self {
        self.params.insert(
            "power".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_power_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "power".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_forpoly(mut self, val: bool) -> Self {
        self.params.insert(
            "forpoly".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forpoly_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forpoly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVCorrugated {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_corrugated"
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
pub struct ShopVDecal {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVDecal {
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

    // --- Float3 parameters ---
    pub fn with_amb(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "amb".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_amb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "diff".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_spec(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spec".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_map(mut self, val: &str) -> Self {
        self.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uwrap(mut self, val: &str) -> Self {
        self.params.insert(
            "uwrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uwrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uwrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vwrap(mut self, val: &str) -> Self {
        self.params.insert(
            "vwrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vwrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vwrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ogl_texuvset2(mut self, val: &str) -> Self {
        self.params.insert(
            "ogl_texuvset2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ogl_texuvset2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ogl_texuvset2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVDecal {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_decal"
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
pub struct ShopVDented {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVDented {
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

    // --- Int parameters ---
    pub fn with_maxoctaves(mut self, val: i32) -> Self {
        self.params.insert(
            "maxoctaves".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxoctaves_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxoctaves".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_for_poly(mut self, val: bool) -> Self {
        self.params.insert(
            "for_poly".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_for_poly_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "for_poly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displace3d(mut self, val: bool) -> Self {
        self.params.insert(
            "displace3d".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displace3d_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displace3d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVDented {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_dented"
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
pub struct ShopVDiffuselighting {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVDiffuselighting {
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
    pub fn with_inf(mut self, val: f32) -> Self {
        self.params.insert(
            "inF".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inF".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instep(mut self, val: f32) -> Self {
        self.params.insert(
            "inStep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_instep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inStep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_now(mut self, val: f32) -> Self {
        self.params.insert(
            "now".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_now_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "now".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_misbias(mut self, val: f32) -> Self {
        self.params.insert(
            "misbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_misbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "misbias".to_string(),
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
    pub fn with_photonspacing(mut self, val: f32) -> Self {
        self.params.insert(
            "photonspacing".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_photonspacing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "photonspacing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_samplingquality(mut self, val: f32) -> Self {
        self.params.insert(
            "samplingquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_samplingquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplingquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rayfilteramount(mut self, val: f32) -> Self {
        self.params.insert(
            "rayfilteramount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rayfilteramount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rayfilteramount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter_width(mut self, val: f32) -> Self {
        self.params.insert(
            "filter_width".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_filter_width_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter_width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter_angle(mut self, val: f32) -> Self {
        self.params.insert(
            "filter_angle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_filter_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter_angle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxrough(mut self, val: f32) -> Self {
        self.params.insert(
            "maxrough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxrough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxrough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_direct_light(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direct_light".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direct_light_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct_light".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inp(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "inP".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_inp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inP".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inps(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "inPs".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_inps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inPs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ini(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "inI".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ini_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inI".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inn(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "inN".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_inn_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inN".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tint(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tint".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_lid(mut self, val: i32) -> Self {
        self.params
            .insert("lid".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_lid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirect_bounces(mut self, val: i32) -> Self {
        self.params.insert(
            "indirect_bounces".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_indirect_bounces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirect_bounces".to_string(),
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
    pub fn with_sid(mut self, val: i32) -> Self {
        self.params
            .insert("sid".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_sid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadow_bounces(mut self, val: i32) -> Self {
        self.params.insert(
            "shadow_bounces".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shadow_bounces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow_bounces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_firstbounce(mut self, val: i32) -> Self {
        self.params.insert(
            "firstbounce".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_firstbounce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "firstbounce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doshadow(mut self, val: i32) -> Self {
        self.params.insert(
            "doshadow".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_doshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_level(mut self, val: i32) -> Self {
        self.params.insert(
            "level".to_string(),
            crate::core::types::ParamValue::Int(val),
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
    pub fn with_ismicropoly(mut self, val: i32) -> Self {
        self.params.insert(
            "ismicropoly".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ismicropoly_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ismicropoly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossytodiffuse(mut self, val: i32) -> Self {
        self.params.insert(
            "glossytodiffuse".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_glossytodiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossytodiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpeid(mut self, val: i32) -> Self {
        self.params.insert(
            "lpeid".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lpeid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpeid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_colorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "colorspace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_colorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpetag_object(mut self, val: &str) -> Self {
        self.params.insert(
            "lpetag_object".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpetag_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpetag_object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVDiffuselighting {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_diffuselighting"
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
pub struct ShopVDistant {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVDistant {
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

    // --- Float3 parameters ---
    pub fn with_lightcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVDistant {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_distant"
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
pub struct ShopVDlayer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVDlayer {
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
    pub fn with_smapblur_base(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur_base".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur_base(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur_base".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount_base(mut self, val: f32) -> Self {
        self.params.insert(
            "amount_base".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur2(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur2(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount2(mut self, val: f32) -> Self {
        self.params.insert(
            "amount2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur3(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur3".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur3(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur3".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount3(mut self, val: f32) -> Self {
        self.params.insert(
            "amount3".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur4(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur4".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur4(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur4".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount4(mut self, val: f32) -> Self {
        self.params.insert(
            "amount4".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur5(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur5".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur5(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur5".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount5(mut self, val: f32) -> Self {
        self.params.insert(
            "amount5".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur6(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur6".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur6(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur6".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount6(mut self, val: f32) -> Self {
        self.params.insert(
            "amount6".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur7(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur7".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur7(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur7".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount7(mut self, val: f32) -> Self {
        self.params.insert(
            "amount7".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur8(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur8".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur8(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur8".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount8(mut self, val: f32) -> Self {
        self.params.insert(
            "amount8".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur9(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur9".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur9(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur9".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount9(mut self, val: f32) -> Self {
        self.params.insert(
            "amount9".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur10(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur10".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur10(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur10".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount10(mut self, val: f32) -> Self {
        self.params.insert(
            "amount10".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur11(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur11".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur11(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur11".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount11(mut self, val: f32) -> Self {
        self.params.insert(
            "amount11".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur12(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur12".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur12(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur12".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount12(mut self, val: f32) -> Self {
        self.params.insert(
            "amount12".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur13(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur13".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur13(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur13".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount13(mut self, val: f32) -> Self {
        self.params.insert(
            "amount13".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur14(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur14".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur14(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur14".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount14(mut self, val: f32) -> Self {
        self.params.insert(
            "amount14".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur15(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur15".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur15(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur15".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount15(mut self, val: f32) -> Self {
        self.params.insert(
            "amount15".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur16(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur16".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur16(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur16".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount16(mut self, val: f32) -> Self {
        self.params.insert(
            "amount16".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amount16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_center_base(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center_base".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate_base(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate_base".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale_base(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale_base".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center4(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate4(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale4(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center5(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center5".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate5(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate5".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale5(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale5".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center6(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center6".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate6(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate6".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale6(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale6".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center7(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center7".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate7(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate7".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale7(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale7".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center8(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center8".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate8(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate8".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale8(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale8".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center9(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center9".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate9(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate9".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale9(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale9".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center10(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center10".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate10(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate10".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale10(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale10".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center11(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center11".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate11(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate11".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale11(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale11".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center12(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center12".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate12(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate12".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale12(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale12".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center13(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center13".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate13(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate13".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale13(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale13".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center14(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center14".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate14(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate14".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale14(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale14".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center15(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center15".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate15(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate15".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale15(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale15".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center16(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center16".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate16(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate16".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale16(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale16".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_map_base(mut self, val: &str) -> Self {
        self.params.insert(
            "map_base".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode_base(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode_base".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter_base(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter_base".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter_base(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter_base".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup_base(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup_base".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project_base(mut self, val: &str) -> Self {
        self.params.insert(
            "project_base".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname_base(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname_base".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space_base(mut self, val: &str) -> Self {
        self.params.insert(
            "space_base".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map2(mut self, val: &str) -> Self {
        self.params.insert(
            "map2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode2(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter2(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter2(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup2(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project2(mut self, val: &str) -> Self {
        self.params.insert(
            "project2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname2(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space2(mut self, val: &str) -> Self {
        self.params.insert(
            "space2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map3(mut self, val: &str) -> Self {
        self.params.insert(
            "map3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode3(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter3(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter3(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup3(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project3(mut self, val: &str) -> Self {
        self.params.insert(
            "project3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname3(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space3(mut self, val: &str) -> Self {
        self.params.insert(
            "space3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map4(mut self, val: &str) -> Self {
        self.params.insert(
            "map4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode4(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter4(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter4(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup4(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project4(mut self, val: &str) -> Self {
        self.params.insert(
            "project4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname4(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space4(mut self, val: &str) -> Self {
        self.params.insert(
            "space4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map5(mut self, val: &str) -> Self {
        self.params.insert(
            "map5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode5(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter5(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter5(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup5(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project5(mut self, val: &str) -> Self {
        self.params.insert(
            "project5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname5(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space5(mut self, val: &str) -> Self {
        self.params.insert(
            "space5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map6(mut self, val: &str) -> Self {
        self.params.insert(
            "map6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode6(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter6(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter6(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup6(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project6(mut self, val: &str) -> Self {
        self.params.insert(
            "project6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname6(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space6(mut self, val: &str) -> Self {
        self.params.insert(
            "space6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map7(mut self, val: &str) -> Self {
        self.params.insert(
            "map7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode7(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter7(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter7(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup7(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project7(mut self, val: &str) -> Self {
        self.params.insert(
            "project7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname7(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space7(mut self, val: &str) -> Self {
        self.params.insert(
            "space7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map8(mut self, val: &str) -> Self {
        self.params.insert(
            "map8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode8(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter8(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter8(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup8(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project8(mut self, val: &str) -> Self {
        self.params.insert(
            "project8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname8(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space8(mut self, val: &str) -> Self {
        self.params.insert(
            "space8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map9(mut self, val: &str) -> Self {
        self.params.insert(
            "map9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode9(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter9(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter9(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup9(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project9(mut self, val: &str) -> Self {
        self.params.insert(
            "project9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname9(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space9(mut self, val: &str) -> Self {
        self.params.insert(
            "space9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map10(mut self, val: &str) -> Self {
        self.params.insert(
            "map10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode10(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter10(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter10(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup10(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project10(mut self, val: &str) -> Self {
        self.params.insert(
            "project10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname10(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space10(mut self, val: &str) -> Self {
        self.params.insert(
            "space10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map11(mut self, val: &str) -> Self {
        self.params.insert(
            "map11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode11(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter11(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter11(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup11(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project11(mut self, val: &str) -> Self {
        self.params.insert(
            "project11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname11(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space11(mut self, val: &str) -> Self {
        self.params.insert(
            "space11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map12(mut self, val: &str) -> Self {
        self.params.insert(
            "map12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode12(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter12(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter12(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup12(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project12(mut self, val: &str) -> Self {
        self.params.insert(
            "project12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname12(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space12(mut self, val: &str) -> Self {
        self.params.insert(
            "space12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map13(mut self, val: &str) -> Self {
        self.params.insert(
            "map13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode13(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter13(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter13(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup13(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project13(mut self, val: &str) -> Self {
        self.params.insert(
            "project13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname13(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space13(mut self, val: &str) -> Self {
        self.params.insert(
            "space13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map14(mut self, val: &str) -> Self {
        self.params.insert(
            "map14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode14(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter14(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter14(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup14(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project14(mut self, val: &str) -> Self {
        self.params.insert(
            "project14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname14(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space14(mut self, val: &str) -> Self {
        self.params.insert(
            "space14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map15(mut self, val: &str) -> Self {
        self.params.insert(
            "map15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode15(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter15(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter15(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup15(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project15(mut self, val: &str) -> Self {
        self.params.insert(
            "project15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname15(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space15(mut self, val: &str) -> Self {
        self.params.insert(
            "space15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map16(mut self, val: &str) -> Self {
        self.params.insert(
            "map16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode16(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter16(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter16(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup16(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project16(mut self, val: &str) -> Self {
        self.params.insert(
            "project16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname16(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space16(mut self, val: &str) -> Self {
        self.params.insert(
            "space16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_dorecompute(mut self, val: bool) -> Self {
        self.params.insert(
            "dorecompute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorecompute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dorecompute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_forpoly(mut self, val: bool) -> Self {
        self.params.insert(
            "forpoly".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forpoly_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forpoly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extra(mut self, val: bool) -> Self {
        self.params.insert(
            "extra".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_extra_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extra".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVDlayer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_dlayer"
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
pub struct ShopVDmap {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVDmap {
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

    // --- String parameters ---
    pub fn with_map(mut self, val: &str) -> Self {
        self.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wrap(mut self, val: &str) -> Self {
        self.params.insert(
            "wrap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_wrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter(mut self, val: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_forpoly(mut self, val: bool) -> Self {
        self.params.insert(
            "forpoly".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forpoly_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forpoly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVDmap {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_dmap"
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
pub struct ShopVFastzmap {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVFastzmap {
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
    pub fn with_attenuation(mut self, val: f32) -> Self {
        self.params.insert(
            "attenuation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_attenuation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attenuation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quality(mut self, val: f32) -> Self {
        self.params.insert(
            "quality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_quality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spread(mut self, val: f32) -> Self {
        self.params.insert(
            "spread".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spread_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spread".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_focal(mut self, val: f32) -> Self {
        self.params.insert(
            "focal".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_focal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "focal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aperture(mut self, val: f32) -> Self {
        self.params.insert(
            "aperture".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aperture_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aperture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_near(mut self, val: f32) -> Self {
        self.params.insert(
            "near".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_near_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "near".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_far(mut self, val: f32) -> Self {
        self.params.insert(
            "far".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_far_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "far".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxsteps(mut self, val: f32) -> Self {
        self.params.insert(
            "maxsteps".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxsteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitter(mut self, val: f32) -> Self {
        self.params.insert(
            "jitter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jitter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jitter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noiseamount(mut self, val: f32) -> Self {
        self.params.insert(
            "noiseamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_noiseamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noiseamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noiserough(mut self, val: f32) -> Self {
        self.params.insert(
            "noiserough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_noiserough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noiserough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noiseatten(mut self, val: f32) -> Self {
        self.params.insert(
            "noiseatten".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_noiseatten_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noiseatten".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_fogclr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "fogclr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fogclr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fogclr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noisefreq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "noisefreq".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_noisefreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noisefreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noiseoff(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "noiseoff".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_noiseoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noiseoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_noiseturb(mut self, val: i32) -> Self {
        self.params.insert(
            "noiseturb".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_noiseturb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noiseturb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_zmap(mut self, val: &str) -> Self {
        self.params.insert(
            "zmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_zmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "zmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pmap(mut self, val: &str) -> Self {
        self.params.insert(
            "pmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noisetype(mut self, val: &str) -> Self {
        self.params.insert(
            "noisetype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_noisetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noisetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noisespace(mut self, val: &str) -> Self {
        self.params.insert(
            "noisespace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_noisespace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noisespace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_inshadow(mut self, val: bool) -> Self {
        self.params.insert(
            "inshadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVFastzmap {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_fastzmap"
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
pub struct ShopVFbx {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVFbx {
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
    pub fn with_emission_mult(mut self, val: f32) -> Self {
        self.params.insert(
            "emission_mult".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_emission_mult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emission_mult".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ambient_mult(mut self, val: f32) -> Self {
        self.params.insert(
            "ambient_mult".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ambient_mult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ambient_mult".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffuse_mult(mut self, val: f32) -> Self {
        self.params.insert(
            "diffuse_mult".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diffuse_mult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffuse_mult".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bump_height(mut self, val: f32) -> Self {
        self.params.insert(
            "bump_height".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bump_height_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bump_height".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opacity_mult(mut self, val: f32) -> Self {
        self.params.insert(
            "opacity_mult".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_opacity_mult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacity_mult".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refl_mult(mut self, val: f32) -> Self {
        self.params.insert(
            "refl_mult".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refl_mult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refl_mult".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specular_mult(mut self, val: f32) -> Self {
        self.params.insert(
            "specular_mult".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_specular_mult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specular_mult".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shininess(mut self, val: f32) -> Self {
        self.params.insert(
            "shininess".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shininess_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shininess".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha1(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha1".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha2(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha3(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha3".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha4(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha4".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha5(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha5".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha6(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha6".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha7(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha7".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha8(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha8".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha9(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha9".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha10(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha10".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha11(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha11".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha12(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha12".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha13(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha13".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha14(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha14".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha15(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha15".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha16(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha16".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_ca(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opacity(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "opacity".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_opacity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opacity".to_string(),
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
    pub fn with_cs(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset1(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset1".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate1(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate1".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale1(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale1".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset4(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate4(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale4(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset5(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset5".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate5(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate5".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale5(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale5".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset6(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset6".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate6(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate6".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale6(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale6".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset7(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset7".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate7(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate7".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale7(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale7".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset8(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset8".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate8(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate8".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale8(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale8".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset9(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset9".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate9(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate9".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale9(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale9".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset10(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset10".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate10(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate10".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale10(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale10".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset11(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset11".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate11(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate11".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale11(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale11".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset12(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset12".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate12(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate12".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale12(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale12".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset13(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset13".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate13(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate13".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale13(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale13".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset14(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset14".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate14(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate14".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale14(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale14".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset15(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset15".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate15(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate15".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale15(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale15".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset16(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset16".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate16(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate16".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale16(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale16".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale16".to_string(),
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

    // --- String parameters ---
    pub fn with_lmodel(mut self, val: &str) -> Self {
        self.params.insert(
            "lmodel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp1(mut self, val: &str) -> Self {
        self.params.insert(
            "comp1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply1(mut self, val: &str) -> Self {
        self.params.insert(
            "apply1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map1(mut self, val: &str) -> Self {
        self.params.insert(
            "map1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname1(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode1(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp2(mut self, val: &str) -> Self {
        self.params.insert(
            "comp2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply2(mut self, val: &str) -> Self {
        self.params.insert(
            "apply2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map2(mut self, val: &str) -> Self {
        self.params.insert(
            "map2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname2(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode2(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp3(mut self, val: &str) -> Self {
        self.params.insert(
            "comp3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply3(mut self, val: &str) -> Self {
        self.params.insert(
            "apply3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map3(mut self, val: &str) -> Self {
        self.params.insert(
            "map3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname3(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode3(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp4(mut self, val: &str) -> Self {
        self.params.insert(
            "comp4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply4(mut self, val: &str) -> Self {
        self.params.insert(
            "apply4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map4(mut self, val: &str) -> Self {
        self.params.insert(
            "map4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname4(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode4(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp5(mut self, val: &str) -> Self {
        self.params.insert(
            "comp5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply5(mut self, val: &str) -> Self {
        self.params.insert(
            "apply5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map5(mut self, val: &str) -> Self {
        self.params.insert(
            "map5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname5(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode5(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp6(mut self, val: &str) -> Self {
        self.params.insert(
            "comp6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply6(mut self, val: &str) -> Self {
        self.params.insert(
            "apply6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map6(mut self, val: &str) -> Self {
        self.params.insert(
            "map6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname6(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode6(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp7(mut self, val: &str) -> Self {
        self.params.insert(
            "comp7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply7(mut self, val: &str) -> Self {
        self.params.insert(
            "apply7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map7(mut self, val: &str) -> Self {
        self.params.insert(
            "map7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname7(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode7(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp8(mut self, val: &str) -> Self {
        self.params.insert(
            "comp8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply8(mut self, val: &str) -> Self {
        self.params.insert(
            "apply8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map8(mut self, val: &str) -> Self {
        self.params.insert(
            "map8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname8(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode8(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp9(mut self, val: &str) -> Self {
        self.params.insert(
            "comp9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply9(mut self, val: &str) -> Self {
        self.params.insert(
            "apply9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map9(mut self, val: &str) -> Self {
        self.params.insert(
            "map9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname9(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode9(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp10(mut self, val: &str) -> Self {
        self.params.insert(
            "comp10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply10(mut self, val: &str) -> Self {
        self.params.insert(
            "apply10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map10(mut self, val: &str) -> Self {
        self.params.insert(
            "map10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname10(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode10(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp11(mut self, val: &str) -> Self {
        self.params.insert(
            "comp11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply11(mut self, val: &str) -> Self {
        self.params.insert(
            "apply11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map11(mut self, val: &str) -> Self {
        self.params.insert(
            "map11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname11(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode11(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp12(mut self, val: &str) -> Self {
        self.params.insert(
            "comp12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply12(mut self, val: &str) -> Self {
        self.params.insert(
            "apply12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map12(mut self, val: &str) -> Self {
        self.params.insert(
            "map12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname12(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode12(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp13(mut self, val: &str) -> Self {
        self.params.insert(
            "comp13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply13(mut self, val: &str) -> Self {
        self.params.insert(
            "apply13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map13(mut self, val: &str) -> Self {
        self.params.insert(
            "map13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname13(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode13(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp14(mut self, val: &str) -> Self {
        self.params.insert(
            "comp14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply14(mut self, val: &str) -> Self {
        self.params.insert(
            "apply14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map14(mut self, val: &str) -> Self {
        self.params.insert(
            "map14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname14(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode14(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp15(mut self, val: &str) -> Self {
        self.params.insert(
            "comp15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply15(mut self, val: &str) -> Self {
        self.params.insert(
            "apply15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map15(mut self, val: &str) -> Self {
        self.params.insert(
            "map15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname15(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode15(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp16(mut self, val: &str) -> Self {
        self.params.insert(
            "comp16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply16(mut self, val: &str) -> Self {
        self.params.insert(
            "apply16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map16(mut self, val: &str) -> Self {
        self.params.insert(
            "map16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname16(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode16(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_swapuv1(mut self, val: bool) -> Self {
        self.params.insert(
            "swapuv1".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapuv1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapuv1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverttexture1(mut self, val: bool) -> Self {
        self.params.insert(
            "inverttexture1".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverttexture1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverttexture1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapuv2(mut self, val: bool) -> Self {
        self.params.insert(
            "swapuv2".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapuv2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapuv2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverttexture2(mut self, val: bool) -> Self {
        self.params.insert(
            "inverttexture2".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverttexture2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverttexture2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapuv3(mut self, val: bool) -> Self {
        self.params.insert(
            "swapuv3".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapuv3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapuv3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverttexture3(mut self, val: bool) -> Self {
        self.params.insert(
            "inverttexture3".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverttexture3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverttexture3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapuv4(mut self, val: bool) -> Self {
        self.params.insert(
            "swapuv4".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapuv4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapuv4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverttexture4(mut self, val: bool) -> Self {
        self.params.insert(
            "inverttexture4".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverttexture4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverttexture4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapuv5(mut self, val: bool) -> Self {
        self.params.insert(
            "swapuv5".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapuv5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapuv5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverttexture5(mut self, val: bool) -> Self {
        self.params.insert(
            "inverttexture5".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverttexture5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverttexture5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapuv6(mut self, val: bool) -> Self {
        self.params.insert(
            "swapuv6".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapuv6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapuv6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverttexture6(mut self, val: bool) -> Self {
        self.params.insert(
            "inverttexture6".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverttexture6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverttexture6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapuv7(mut self, val: bool) -> Self {
        self.params.insert(
            "swapuv7".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapuv7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapuv7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverttexture7(mut self, val: bool) -> Self {
        self.params.insert(
            "inverttexture7".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverttexture7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverttexture7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapuv8(mut self, val: bool) -> Self {
        self.params.insert(
            "swapuv8".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapuv8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapuv8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverttexture8(mut self, val: bool) -> Self {
        self.params.insert(
            "inverttexture8".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverttexture8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverttexture8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapuv9(mut self, val: bool) -> Self {
        self.params.insert(
            "swapuv9".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapuv9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapuv9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverttexture9(mut self, val: bool) -> Self {
        self.params.insert(
            "inverttexture9".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverttexture9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverttexture9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapuv10(mut self, val: bool) -> Self {
        self.params.insert(
            "swapuv10".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapuv10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapuv10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverttexture10(mut self, val: bool) -> Self {
        self.params.insert(
            "inverttexture10".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverttexture10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverttexture10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapuv11(mut self, val: bool) -> Self {
        self.params.insert(
            "swapuv11".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapuv11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapuv11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverttexture11(mut self, val: bool) -> Self {
        self.params.insert(
            "inverttexture11".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverttexture11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverttexture11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapuv12(mut self, val: bool) -> Self {
        self.params.insert(
            "swapuv12".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapuv12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapuv12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverttexture12(mut self, val: bool) -> Self {
        self.params.insert(
            "inverttexture12".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverttexture12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverttexture12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapuv13(mut self, val: bool) -> Self {
        self.params.insert(
            "swapuv13".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapuv13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapuv13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverttexture13(mut self, val: bool) -> Self {
        self.params.insert(
            "inverttexture13".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverttexture13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverttexture13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapuv14(mut self, val: bool) -> Self {
        self.params.insert(
            "swapuv14".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapuv14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapuv14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverttexture14(mut self, val: bool) -> Self {
        self.params.insert(
            "inverttexture14".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverttexture14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverttexture14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapuv15(mut self, val: bool) -> Self {
        self.params.insert(
            "swapuv15".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapuv15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapuv15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverttexture15(mut self, val: bool) -> Self {
        self.params.insert(
            "inverttexture15".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverttexture15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverttexture15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swapuv16(mut self, val: bool) -> Self {
        self.params.insert(
            "swapuv16".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapuv16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapuv16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverttexture16(mut self, val: bool) -> Self {
        self.params.insert(
            "inverttexture16".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverttexture16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverttexture16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVFbx {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_fbx"
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
pub struct ShopVFluffy {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVFluffy {
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
    pub fn with_fringe(mut self, val: f32) -> Self {
        self.params.insert(
            "fringe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fringe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fringe".to_string(),
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
    pub fn with_amp(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "amp".to_string(),
            crate::core::types::ParamValue::Float3(val),
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

    // --- Int parameters ---
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
}

impl crate::core::types::HoudiniNode for ShopVFluffy {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_fluffy"
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
pub struct ShopVGilight {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVGilight {
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
    pub fn with_cone_angle(mut self, val: f32) -> Self {
        self.params.insert(
            "cone_angle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cone_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cone_angle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxdist(mut self, val: f32) -> Self {
        self.params.insert(
            "maxdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_threshold(mut self, val: f32) -> Self {
        self.params.insert(
            "threshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_threshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "threshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_light_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "light_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_light_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_envtint(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "envtint".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_envtint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envtint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_background(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "background".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_background_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "background".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "mapsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mapsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minlevel(mut self, val: i32) -> Self {
        self.params.insert(
            "minlevel".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_minlevel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minlevel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_istyle(mut self, val: &str) -> Self {
        self.params.insert(
            "istyle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_istyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "istyle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_objmask(mut self, val: &str) -> Self {
        self.params.insert(
            "objmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_objmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_envmap(mut self, val: &str) -> Self {
        self.params.insert(
            "envmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_envmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_envnull(mut self, val: &str) -> Self {
        self.params.insert(
            "envnull".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_envnull_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envnull".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapfile(mut self, val: &str) -> Self {
        self.params.insert(
            "mapfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter(mut self, val: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_doraysamples(mut self, val: bool) -> Self {
        self.params.insert(
            "doraysamples".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doraysamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doraysamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doadaptive(mut self, val: bool) -> Self {
        self.params.insert(
            "doadaptive".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doadaptive_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doadaptive".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domaxdist(mut self, val: bool) -> Self {
        self.params.insert(
            "domaxdist".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domaxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domaxdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doobjmask(mut self, val: bool) -> Self {
        self.params.insert(
            "doobjmask".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doobjmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doobjmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dobackground(mut self, val: bool) -> Self {
        self.params.insert(
            "dobackground".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dobackground_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dobackground".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefilter(mut self, val: bool) -> Self {
        self.params.insert(
            "prefilter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachedirect(mut self, val: bool) -> Self {
        self.params.insert(
            "cachedirect".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cachedirect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachedirect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVGilight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_gilight"
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
pub struct ShopVGingham {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVGingham {
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
    pub fn with_size(mut self, val: f32) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Float(val),
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

    // --- Float3 parameters ---
    pub fn with_blue(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "blue".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_blue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_white(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "white".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_white_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "white".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVGingham {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_gingham"
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
pub struct ShopVGlass {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVGlass {
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
    pub fn with_cone_angle(mut self, val: f32) -> Self {
        self.params.insert(
            "cone_angle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cone_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cone_angle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitter(mut self, val: f32) -> Self {
        self.params.insert(
            "jitter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jitter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jitter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_amb(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "amb".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_amb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "diff".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_spec(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spec".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec".to_string(),
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
    pub fn with_transmit(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "transmit".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_transmit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transmit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_atten(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "atten".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_maprefl(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "maprefl".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_maprefl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maprefl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_area_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "area_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_area_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "area_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_envmap(mut self, val: &str) -> Self {
        self.params.insert(
            "envmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_envmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVGlass {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_glass"
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
pub struct ShopVHair {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVHair {
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
    pub fn with_ka(mut self, val: f32) -> Self {
        self.params
            .insert("Ka".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ka_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ka".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_shinyness(mut self, val: f32) -> Self {
        self.params.insert(
            "shinyness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shinyness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shinyness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_tip_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tip_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tip_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tip_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_root_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "root_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_root_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "root_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tip_opac(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tip_opac".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tip_opac_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tip_opac".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_root_opac(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "root_opac".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_root_opac_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "root_opac".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spec_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spec_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spec_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_doshadow(mut self, val: bool) -> Self {
        self.params.insert(
            "doshadow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVHair {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_hair"
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
pub struct ShopVKrinkle {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVKrinkle {
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

    // --- Toggle parameters ---
    pub fn with_forpoly(mut self, val: bool) -> Self {
        self.params.insert(
            "forpoly".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forpoly_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forpoly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVKrinkle {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_krinkle"
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
pub struct ShopVLayered {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVLayered {
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
    pub fn with_apara(mut self, val: f32) -> Self {
        self.params.insert(
            "apara".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_apara_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apara".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aperp(mut self, val: f32) -> Self {
        self.params.insert(
            "aperp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aperp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aperp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aroll(mut self, val: f32) -> Self {
        self.params.insert(
            "aroll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aroll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aroll".to_string(),
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
    pub fn with_fresnel_eta(mut self, val: f32) -> Self {
        self.params.insert(
            "fresnel_eta".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fresnel_eta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fresnel_eta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur_base(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur_base".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur_base(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur_base".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur2(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur2(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha2(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur3(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur3".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur3(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur3".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha3(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha3".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur4(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur4".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur4(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur4".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha4(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha4".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur5(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur5".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur5(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur5".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha5(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha5".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur6(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur6".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur6(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur6".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha6(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha6".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur7(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur7".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur7(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur7".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha7(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha7".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur8(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur8".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur8(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur8".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha8(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha8".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur9(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur9".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur9(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur9".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha9(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha9".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur10(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur10".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur10(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur10".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha10(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha10".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur11(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur11".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur11(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur11".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha11(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha11".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur12(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur12".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur12(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur12".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha12(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha12".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur13(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur13".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur13(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur13".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha13(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha13".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur14(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur14".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur14(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur14".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha14(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha14".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur15(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur15".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur15(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur15".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha15(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha15".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapblur16(mut self, val: f32) -> Self {
        self.params.insert(
            "smapblur16".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smapblur16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapblur16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapblur16(mut self, val: f32) -> Self {
        self.params.insert(
            "tmapblur16".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tmapblur16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapblur16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alpha16(mut self, val: f32) -> Self {
        self.params.insert(
            "Alpha16".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Alpha16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_ca(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ct(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ct".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ct_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ct".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center_base(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center_base".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate_base(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate_base".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale_base(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale_base".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ca2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cr2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cr2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cr2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cr2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ca3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cr3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cr3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cr3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cr3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ca4(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd4(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs4(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cr4(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cr4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cr4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cr4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center4(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate4(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale4(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ca5(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca5".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd5(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd5".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs5(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs5".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cr5(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cr5".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cr5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cr5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center5(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center5".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate5(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate5".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale5(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale5".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ca6(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca6".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd6(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd6".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs6(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs6".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cr6(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cr6".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cr6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cr6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center6(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center6".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate6(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate6".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale6(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale6".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ca7(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca7".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd7(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd7".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs7(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs7".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cr7(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cr7".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cr7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cr7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center7(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center7".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate7(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate7".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale7(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale7".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ca8(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca8".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd8(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd8".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs8(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs8".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cr8(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cr8".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cr8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cr8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center8(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center8".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate8(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate8".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale8(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale8".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ca9(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca9".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd9(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd9".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs9(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs9".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cr9(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cr9".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cr9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cr9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center9(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center9".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate9(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate9".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale9(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale9".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ca10(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca10".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd10(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd10".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs10(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs10".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cr10(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cr10".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cr10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cr10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center10(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center10".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate10(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate10".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale10(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale10".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ca11(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca11".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd11(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd11".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs11(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs11".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cr11(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cr11".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cr11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cr11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center11(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center11".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate11(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate11".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale11(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale11".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ca12(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca12".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd12(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd12".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs12(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs12".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cr12(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cr12".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cr12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cr12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center12(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center12".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate12(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate12".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale12(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale12".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ca13(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca13".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd13(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd13".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs13(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs13".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cr13(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cr13".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cr13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cr13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center13(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center13".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate13(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate13".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale13(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale13".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ca14(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca14".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd14(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd14".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs14(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs14".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cr14(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cr14".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cr14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cr14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center14(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center14".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate14(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate14".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale14(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale14".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ca15(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca15".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd15(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd15".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs15(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs15".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cr15(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cr15".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cr15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cr15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center15(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center15".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate15(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate15".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale15(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale15".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ca16(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Ca16".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ca16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ca16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cd16(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd16".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cs16(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cs16".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cs16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cs16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cr16(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cr16".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cr16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cr16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_center16(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center16".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate16(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate16".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotate16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotate16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale16(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "scale16".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scale16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_lmodel(mut self, val: &str) -> Self {
        self.params.insert(
            "lmodel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map_base(mut self, val: &str) -> Self {
        self.params.insert(
            "map_base".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply_base(mut self, val: &str) -> Self {
        self.params.insert(
            "apply_base".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode_base(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode_base".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter_base(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter_base".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter_base(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter_base".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project_base(mut self, val: &str) -> Self {
        self.params.insert(
            "project_base".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname_base(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname_base".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space_base(mut self, val: &str) -> Self {
        self.params.insert(
            "space_base".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space_base_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space_base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp2(mut self, val: &str) -> Self {
        self.params.insert(
            "comp2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map2(mut self, val: &str) -> Self {
        self.params.insert(
            "map2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply2(mut self, val: &str) -> Self {
        self.params.insert(
            "apply2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode2(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter2(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter2(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project2(mut self, val: &str) -> Self {
        self.params.insert(
            "project2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname2(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space2(mut self, val: &str) -> Self {
        self.params.insert(
            "space2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp3(mut self, val: &str) -> Self {
        self.params.insert(
            "comp3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map3(mut self, val: &str) -> Self {
        self.params.insert(
            "map3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply3(mut self, val: &str) -> Self {
        self.params.insert(
            "apply3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode3(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter3(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter3(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project3(mut self, val: &str) -> Self {
        self.params.insert(
            "project3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname3(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space3(mut self, val: &str) -> Self {
        self.params.insert(
            "space3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp4(mut self, val: &str) -> Self {
        self.params.insert(
            "comp4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map4(mut self, val: &str) -> Self {
        self.params.insert(
            "map4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply4(mut self, val: &str) -> Self {
        self.params.insert(
            "apply4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode4(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter4(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter4(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project4(mut self, val: &str) -> Self {
        self.params.insert(
            "project4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname4(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space4(mut self, val: &str) -> Self {
        self.params.insert(
            "space4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp5(mut self, val: &str) -> Self {
        self.params.insert(
            "comp5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map5(mut self, val: &str) -> Self {
        self.params.insert(
            "map5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply5(mut self, val: &str) -> Self {
        self.params.insert(
            "apply5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode5(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter5(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter5(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project5(mut self, val: &str) -> Self {
        self.params.insert(
            "project5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname5(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space5(mut self, val: &str) -> Self {
        self.params.insert(
            "space5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp6(mut self, val: &str) -> Self {
        self.params.insert(
            "comp6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map6(mut self, val: &str) -> Self {
        self.params.insert(
            "map6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply6(mut self, val: &str) -> Self {
        self.params.insert(
            "apply6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode6(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter6(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter6(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project6(mut self, val: &str) -> Self {
        self.params.insert(
            "project6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname6(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space6(mut self, val: &str) -> Self {
        self.params.insert(
            "space6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp7(mut self, val: &str) -> Self {
        self.params.insert(
            "comp7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map7(mut self, val: &str) -> Self {
        self.params.insert(
            "map7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply7(mut self, val: &str) -> Self {
        self.params.insert(
            "apply7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode7(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter7(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter7(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project7(mut self, val: &str) -> Self {
        self.params.insert(
            "project7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname7(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space7(mut self, val: &str) -> Self {
        self.params.insert(
            "space7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp8(mut self, val: &str) -> Self {
        self.params.insert(
            "comp8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map8(mut self, val: &str) -> Self {
        self.params.insert(
            "map8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply8(mut self, val: &str) -> Self {
        self.params.insert(
            "apply8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode8(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter8(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter8(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project8(mut self, val: &str) -> Self {
        self.params.insert(
            "project8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname8(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space8(mut self, val: &str) -> Self {
        self.params.insert(
            "space8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp9(mut self, val: &str) -> Self {
        self.params.insert(
            "comp9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map9(mut self, val: &str) -> Self {
        self.params.insert(
            "map9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply9(mut self, val: &str) -> Self {
        self.params.insert(
            "apply9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode9(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter9(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter9(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project9(mut self, val: &str) -> Self {
        self.params.insert(
            "project9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname9(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space9(mut self, val: &str) -> Self {
        self.params.insert(
            "space9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp10(mut self, val: &str) -> Self {
        self.params.insert(
            "comp10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map10(mut self, val: &str) -> Self {
        self.params.insert(
            "map10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply10(mut self, val: &str) -> Self {
        self.params.insert(
            "apply10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode10(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter10(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter10(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project10(mut self, val: &str) -> Self {
        self.params.insert(
            "project10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname10(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space10(mut self, val: &str) -> Self {
        self.params.insert(
            "space10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp11(mut self, val: &str) -> Self {
        self.params.insert(
            "comp11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map11(mut self, val: &str) -> Self {
        self.params.insert(
            "map11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply11(mut self, val: &str) -> Self {
        self.params.insert(
            "apply11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode11(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter11(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter11(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project11(mut self, val: &str) -> Self {
        self.params.insert(
            "project11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname11(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space11(mut self, val: &str) -> Self {
        self.params.insert(
            "space11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp12(mut self, val: &str) -> Self {
        self.params.insert(
            "comp12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map12(mut self, val: &str) -> Self {
        self.params.insert(
            "map12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply12(mut self, val: &str) -> Self {
        self.params.insert(
            "apply12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode12(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter12(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter12(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project12(mut self, val: &str) -> Self {
        self.params.insert(
            "project12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname12(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space12(mut self, val: &str) -> Self {
        self.params.insert(
            "space12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp13(mut self, val: &str) -> Self {
        self.params.insert(
            "comp13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map13(mut self, val: &str) -> Self {
        self.params.insert(
            "map13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply13(mut self, val: &str) -> Self {
        self.params.insert(
            "apply13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode13(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter13(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter13(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project13(mut self, val: &str) -> Self {
        self.params.insert(
            "project13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname13(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space13(mut self, val: &str) -> Self {
        self.params.insert(
            "space13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space13_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp14(mut self, val: &str) -> Self {
        self.params.insert(
            "comp14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map14(mut self, val: &str) -> Self {
        self.params.insert(
            "map14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply14(mut self, val: &str) -> Self {
        self.params.insert(
            "apply14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode14(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter14(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter14(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project14(mut self, val: &str) -> Self {
        self.params.insert(
            "project14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname14(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space14(mut self, val: &str) -> Self {
        self.params.insert(
            "space14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space14_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp15(mut self, val: &str) -> Self {
        self.params.insert(
            "comp15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map15(mut self, val: &str) -> Self {
        self.params.insert(
            "map15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply15(mut self, val: &str) -> Self {
        self.params.insert(
            "apply15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode15(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter15(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter15(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project15(mut self, val: &str) -> Self {
        self.params.insert(
            "project15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname15(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space15(mut self, val: &str) -> Self {
        self.params.insert(
            "space15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space15_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_comp16(mut self, val: &str) -> Self {
        self.params.insert(
            "comp16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comp16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comp16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map16(mut self, val: &str) -> Self {
        self.params.insert(
            "map16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply16(mut self, val: &str) -> Self {
        self.params.insert(
            "apply16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apply16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapmode16(mut self, val: &str) -> Self {
        self.params.insert(
            "mapmode16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapmode16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapmode16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smapfilter16(mut self, val: &str) -> Self {
        self.params.insert(
            "smapfilter16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_smapfilter16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smapfilter16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tmapfilter16(mut self, val: &str) -> Self {
        self.params.insert(
            "tmapfilter16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tmapfilter16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tmapfilter16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_project16(mut self, val: &str) -> Self {
        self.params.insert(
            "project16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_project16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "project16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvname16(mut self, val: &str) -> Self {
        self.params.insert(
            "uvname16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvname16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvname16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_space16(mut self, val: &str) -> Self {
        self.params.insert(
            "space16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "space16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_tintedambient(mut self, val: bool) -> Self {
        self.params.insert(
            "tintedambient".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tintedambient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tintedambient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dofresnel(mut self, val: bool) -> Self {
        self.params.insert(
            "dofresnel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dofresnel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dofresnel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVLayered {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_layered"
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
pub struct ShopVLighttracer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVLighttracer {
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

    // --- Int parameters ---
    pub fn with_transparentsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "transparentsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_transparentsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transparentsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limit(mut self, val: i32) -> Self {
        self.params.insert(
            "limit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_limit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_target(mut self, val: &str) -> Self {
        self.params.insert(
            "target".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_target_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "target".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map(mut self, val: &str) -> Self {
        self.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_stochastic(mut self, val: bool) -> Self {
        self.params.insert(
            "stochastic".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stochastic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stochastic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onlycaustics(mut self, val: bool) -> Self {
        self.params.insert(
            "onlycaustics".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_onlycaustics_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "onlycaustics".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alllights(mut self, val: bool) -> Self {
        self.params.insert(
            "alllights".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alllights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alllights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVLighttracer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_lighttracer"
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
pub struct ShopVLitfog {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVLitfog {
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
    pub fn with_stepsize(mut self, val: f32) -> Self {
        self.params.insert(
            "stepsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stepsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stepsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_n_amp(mut self, val: f32) -> Self {
        self.params.insert(
            "n_amp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_n_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "n_amp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_n_rough(mut self, val: f32) -> Self {
        self.params.insert(
            "n_rough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_n_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "n_rough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "clr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_n_freq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "n_freq".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_n_freq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "n_freq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_n_off(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "n_off".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_n_off_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "n_off".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_maxsteps(mut self, val: i32) -> Self {
        self.params.insert(
            "maxsteps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxsteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_n_octaves(mut self, val: i32) -> Self {
        self.params.insert(
            "n_octaves".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_n_octaves_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "n_octaves".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_doshadows(mut self, val: bool) -> Self {
        self.params.insert(
            "doshadows".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doshadows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doshadows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVLitfog {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_litfog"
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
pub struct ShopVMatte {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVMatte {
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
    pub fn with_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha".to_string(),
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
    pub fn with_clouddensity(mut self, val: f32) -> Self {
        self.params.insert(
            "clouddensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clouddensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clouddensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "shadowdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadowdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_nofog(mut self, val: bool) -> Self {
        self.params.insert(
            "__nofog".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_nofog_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "__nofog".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVMatte {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_matte"
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
pub struct ShopVMetacloud {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVMetacloud {
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
    pub fn with_limit(mut self, val: f32) -> Self {
        self.params.insert(
            "limit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_limit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limit".to_string(),
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
    pub fn with_amp(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "amp".to_string(),
            crate::core::types::ParamValue::Float3(val),
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

    // --- Int parameters ---
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

    // --- String parameters ---
    pub fn with_summing(mut self, val: &str) -> Self {
        self.params.insert(
            "summing".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_summing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "summing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_perpoint(mut self, val: bool) -> Self {
        self.params.insert(
            "perpoint".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_perpoint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "perpoint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVMetacloud {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_metacloud"
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
pub struct ShopVMetal {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVMetal {
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
    pub fn with_cone_angle(mut self, val: f32) -> Self {
        self.params.insert(
            "cone_angle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cone_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cone_angle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_envblur(mut self, val: f32) -> Self {
        self.params.insert(
            "envblur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_envblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_amb(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "amb".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_amb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "diff".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_spec(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spec".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec".to_string(),
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
    pub fn with_envtint(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "envtint".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_envtint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envtint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_area_samples(mut self, val: i32) -> Self {
        self.params.insert(
            "area_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_area_samples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "area_samples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_envmap(mut self, val: &str) -> Self {
        self.params.insert(
            "envmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_envmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_envnull(mut self, val: &str) -> Self {
        self.params.insert(
            "envnull".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_envnull_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "envnull".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVMetal {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_metal"
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
pub struct ShopVMislighting {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVMislighting {
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
    pub fn with_inf(mut self, val: f32) -> Self {
        self.params.insert(
            "inF".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inF".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instep(mut self, val: f32) -> Self {
        self.params.insert(
            "inStep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_instep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inStep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_now(mut self, val: f32) -> Self {
        self.params.insert(
            "now".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_now_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "now".to_string(),
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
    pub fn with_photonspacing(mut self, val: f32) -> Self {
        self.params.insert(
            "photonspacing".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_photonspacing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "photonspacing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_samplingquality(mut self, val: f32) -> Self {
        self.params.insert(
            "samplingquality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_samplingquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplingquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rayfilteramount(mut self, val: f32) -> Self {
        self.params.insert(
            "rayfilteramount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rayfilteramount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rayfilteramount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter_width(mut self, val: f32) -> Self {
        self.params.insert(
            "filter_width".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_filter_width_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter_width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter_angle(mut self, val: f32) -> Self {
        self.params.insert(
            "filter_angle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_filter_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter_angle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_misbias(mut self, val: f32) -> Self {
        self.params.insert(
            "misbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_misbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "misbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filteramount(mut self, val: f32) -> Self {
        self.params.insert(
            "filteramount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_filteramount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filteramount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxrough(mut self, val: f32) -> Self {
        self.params.insert(
            "maxrough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxrough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxrough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_direct_light(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direct_light".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direct_light_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direct_light".to_string(),
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
    pub fn with_inp(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "inP".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_inp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inP".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inps(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "inPs".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_inps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inPs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ini(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "inI".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ini_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inI".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inn(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "inN".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_inn_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inN".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tint(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tint".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_lid(mut self, val: i32) -> Self {
        self.params
            .insert("lid".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_lid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirect_bounces(mut self, val: i32) -> Self {
        self.params.insert(
            "indirect_bounces".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_indirect_bounces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirect_bounces".to_string(),
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
    pub fn with_sid(mut self, val: i32) -> Self {
        self.params
            .insert("sid".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_sid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadow_bounces(mut self, val: i32) -> Self {
        self.params.insert(
            "shadow_bounces".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shadow_bounces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow_bounces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_firstbounce(mut self, val: i32) -> Self {
        self.params.insert(
            "firstbounce".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_firstbounce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "firstbounce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doshadow(mut self, val: i32) -> Self {
        self.params.insert(
            "doshadow".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_doshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_level(mut self, val: i32) -> Self {
        self.params.insert(
            "level".to_string(),
            crate::core::types::ParamValue::Int(val),
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
    pub fn with_dovariance(mut self, val: i32) -> Self {
        self.params.insert(
            "dovariance".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_dovariance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dovariance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ismicropoly(mut self, val: i32) -> Self {
        self.params.insert(
            "ismicropoly".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ismicropoly_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ismicropoly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectlight(mut self, val: i32) -> Self {
        self.params.insert(
            "indirectlight".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_indirectlight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectlight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpeid(mut self, val: i32) -> Self {
        self.params.insert(
            "lpeid".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lpeid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpeid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_colorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "colorspace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_colorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpetag_object(mut self, val: &str) -> Self {
        self.params.insert(
            "lpetag_object".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpetag_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpetag_object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVMislighting {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_mislighting"
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
pub struct ShopVOglmaterial {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVOglmaterial {
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
    pub fn with_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shiny(mut self, val: f32) -> Self {
        self.params.insert(
            "shiny".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shiny_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shiny".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_amb(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "amb".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_amb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "diff".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_spec(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spec".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emit(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "emit".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_emit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVOglmaterial {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_oglmaterial"
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
pub struct ShopVPathtracer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVPathtracer {
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
    pub fn with_reflectratio(mut self, val: f32) -> Self {
        self.params.insert(
            "reflectratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflectratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflectratio".to_string(),
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
    pub fn with_rayfilteramount(mut self, val: f32) -> Self {
        self.params.insert(
            "rayfilteramount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rayfilteramount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rayfilteramount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
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

    // --- String parameters ---
    pub fn with_pathtype(mut self, val: &str) -> Self {
        self.params.insert(
            "pathtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raylimiteval(mut self, val: &str) -> Self {
        self.params.insert(
            "raylimiteval".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_raylimiteval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raylimiteval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorspace(mut self, val: &str) -> Self {
        self.params.insert(
            "colorspace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_colorspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "colorspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_use_renderstate(mut self, val: bool) -> Self {
        self.params.insert(
            "use_renderstate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_renderstate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_renderstate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_recursive(mut self, val: bool) -> Self {
        self.params.insert(
            "recursive".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_recursive_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "recursive".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVPathtracer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_pathtracer"
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
pub struct ShopVPcwriter {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVPcwriter {
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

    // --- Int parameters ---
    pub fn with_localspace(mut self, val: i32) -> Self {
        self.params.insert(
            "localspace".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_localspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pcfile(mut self, val: &str) -> Self {
        self.params.insert(
            "pcfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pcfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVPcwriter {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_pcwriter"
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
pub struct ShopVPhotontracer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVPhotontracer {
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

    // --- String parameters ---
    pub fn with_gfile(mut self, val: &str) -> Self {
        self.params.insert(
            "gfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_gfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cfile(mut self, val: &str) -> Self {
        self.params.insert(
            "cfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_use_renderstate(mut self, val: bool) -> Self {
        self.params.insert(
            "use_renderstate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_renderstate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_renderstate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachedirect(mut self, val: bool) -> Self {
        self.params.insert(
            "cachedirect".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cachedirect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachedirect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_include_bsdf_color(mut self, val: bool) -> Self {
        self.params.insert(
            "include_bsdf_color".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_include_bsdf_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "include_bsdf_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVPhotontracer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_photontracer"
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
pub struct ShopVPlastic {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVPlastic {
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

    // --- Float3 parameters ---
    pub fn with_amb(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "amb".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_amb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "diff".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_spec(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spec".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVPlastic {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_plastic"
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
pub struct ShopVPointlight {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVPointlight {
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

    // --- Float3 parameters ---
    pub fn with_lightcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVPointlight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_pointlight"
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
pub struct ShopVPolka {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVPolka {
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
    pub fn with_dotsize(mut self, val: f32) -> Self {
        self.params.insert(
            "dotsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dotsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dotsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_softness(mut self, val: f32) -> Self {
        self.params.insert(
            "softness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_softness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "softness".to_string(),
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
    pub fn with_dotclr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dotclr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dotclr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dotclr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_baseclr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "baseclr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_baseclr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "baseclr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specclr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "specclr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_specclr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specclr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVPolka {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_polka"
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
pub struct ShopVPrefilter {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVPrefilter {
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
    pub fn with_ratio(mut self, val: f32) -> Self {
        self.params.insert(
            "ratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_count(mut self, val: i32) -> Self {
        self.params.insert(
            "count".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "count".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ptnum(mut self, val: i32) -> Self {
        self.params.insert(
            "ptnum".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ptnum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ptnum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dim(mut self, val: i32) -> Self {
        self.params
            .insert("dim".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_dim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_photon_file(mut self, val: &str) -> Self {
        self.params.insert(
            "photon_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_photon_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "photon_file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefiltered_file(mut self, val: &str) -> Self {
        self.params.insert(
            "prefiltered_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prefiltered_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefiltered_file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter(mut self, val: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVPrefilter {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_prefilter"
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
pub enum ShopVRayshadowMapMipInterp {
    Off = 0,
    On = 1,
}

#[derive(Debug, Clone)]
pub struct ShopVRayshadow {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVRayshadow {
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
    pub fn with_shadowi(mut self, val: f32) -> Self {
        self.params.insert(
            "shadowI".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadowi_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowI".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bias(mut self, val: f32) -> Self {
        self.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spread(mut self, val: f32) -> Self {
        self.params.insert(
            "spread".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spread_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spread".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quality(mut self, val: f32) -> Self {
        self.params.insert(
            "quality".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_quality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blur(mut self, val: f32) -> Self {
        self.params.insert(
            "blur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_shadow_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shadow_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shadow_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadow_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map_border(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "map_border".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_map_border_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map_border".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_map_mip_interp(mut self, val: ShopVRayshadowMapMipInterp) -> Self {
        self.params.insert(
            "map_mip_interp".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_map_mip_interp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map_mip_interp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_shadowtype(mut self, val: &str) -> Self {
        self.params.insert(
            "shadowtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shadowtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map(mut self, val: &str) -> Self {
        self.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map_filter(mut self, val: &str) -> Self {
        self.params.insert(
            "map_filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map_filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map_depth_interp(mut self, val: &str) -> Self {
        self.params.insert(
            "map_depth_interp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map_depth_interp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map_depth_interp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_map_time_zero(mut self, val: bool) -> Self {
        self.params.insert(
            "map_time_zero".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_map_time_zero_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "map_time_zero".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVRayshadow {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_rayshadow"
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
pub struct ShopVRiverbed {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVRiverbed {
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

    // --- Toggle parameters ---
    pub fn with_forpoly(mut self, val: bool) -> Self {
        self.params.insert(
            "forpoly".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forpoly_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forpoly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVRiverbed {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_riverbed"
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
pub struct ShopVSamplerGeometry {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVSamplerGeometry {
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
    pub fn with_now(mut self, val: f32) -> Self {
        self.params.insert(
            "now".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_now_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "now".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activeradius(mut self, val: f32) -> Self {
        self.params.insert(
            "activeradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_activeradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activeradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lscale(mut self, val: f32) -> Self {
        self.params.insert(
            "lscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_pos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_sam(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sam".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sam_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sam".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ldir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ldir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ldir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ldir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lclr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lclr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lclr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lclr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_hit(mut self, val: i32) -> Self {
        self.params
            .insert("hit".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_hit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_selfshadow(mut self, val: i32) -> Self {
        self.params.insert(
            "selfshadow".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_selfshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "selfshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVSamplerGeometry {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_sampler_geometry"
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
pub struct ShopVSamplerPclight {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVSamplerPclight {
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
    pub fn with_now(mut self, val: f32) -> Self {
        self.params.insert(
            "now".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_now_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "now".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activeradius(mut self, val: f32) -> Self {
        self.params.insert(
            "activeradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_activeradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activeradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lscale(mut self, val: f32) -> Self {
        self.params.insert(
            "lscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_pos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_sam(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sam".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sam_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sam".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nml(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "nml".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_nml_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nml".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ldir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ldir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ldir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ldir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lclr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lclr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lclr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lclr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_hit(mut self, val: i32) -> Self {
        self.params
            .insert("hit".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_hit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "pcsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pcsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_selfshadow(mut self, val: i32) -> Self {
        self.params.insert(
            "selfshadow".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_selfshadow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "selfshadow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pcfile(mut self, val: &str) -> Self {
        self.params.insert(
            "pcfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pcfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raystyle(mut self, val: &str) -> Self {
        self.params.insert(
            "raystyle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_raystyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "raystyle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVSamplerPclight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_sampler_pclight"
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
pub struct ShopVShadowmatte {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVShadowmatte {
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
    pub fn with_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alpha".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_rmback(mut self, val: bool) -> Self {
        self.params.insert(
            "rmback".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rmback_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rmback".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nofog(mut self, val: bool) -> Self {
        self.params.insert(
            "__nofog".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_nofog_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "__nofog".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVShadowmatte {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_shadowmatte"
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
pub struct ShopVSofttoon {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVSofttoon {
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
    pub fn with_contrast(mut self, val: f32) -> Self {
        self.params.insert(
            "contrast".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_contrast_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contrast".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_cd(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVSofttoon {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_softtoon"
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
pub struct ShopVSupermat {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVSupermat {
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
    pub fn with_drough(mut self, val: f32) -> Self {
        self.params.insert(
            "drough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_drough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "drough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srough(mut self, val: f32) -> Self {
        self.params.insert(
            "srough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_srough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srough".to_string(),
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
    pub fn with_refract(mut self, val: f32) -> Self {
        self.params.insert(
            "refract".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refract_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refract".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apara(mut self, val: f32) -> Self {
        self.params.insert(
            "apara".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_apara_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apara".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aperp(mut self, val: f32) -> Self {
        self.params.insert(
            "aperp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aperp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aperp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aroll(mut self, val: f32) -> Self {
        self.params.insert(
            "aroll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aroll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aroll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ambcoef(mut self, val: f32) -> Self {
        self.params.insert(
            "ambcoef".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ambcoef_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ambcoef".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffcoef(mut self, val: f32) -> Self {
        self.params.insert(
            "diffcoef".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_diffcoef_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffcoef".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_speccoef(mut self, val: f32) -> Self {
        self.params.insert(
            "speccoef".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_speccoef_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speccoef".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflcoef(mut self, val: f32) -> Self {
        self.params.insert(
            "reflcoef".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflcoef_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflcoef".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transcoef(mut self, val: f32) -> Self {
        self.params.insert(
            "transcoef".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_transcoef_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transcoef".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emitcoef(mut self, val: f32) -> Self {
        self.params.insert(
            "emitcoef".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_emitcoef_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emitcoef".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_droughcoef(mut self, val: f32) -> Self {
        self.params.insert(
            "droughcoef".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_droughcoef_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "droughcoef".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sroughcoef(mut self, val: f32) -> Self {
        self.params.insert(
            "sroughcoef".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sroughcoef_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sroughcoef".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractcoef(mut self, val: f32) -> Self {
        self.params.insert(
            "refractcoef".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refractcoef_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractcoef".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aparacoef(mut self, val: f32) -> Self {
        self.params.insert(
            "aparacoef".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aparacoef_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aparacoef".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aperpcoef(mut self, val: f32) -> Self {
        self.params.insert(
            "aperpcoef".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_aperpcoef_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aperpcoef".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_arollcoef(mut self, val: f32) -> Self {
        self.params.insert(
            "arollcoef".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_arollcoef_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "arollcoef".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowcoef(mut self, val: f32) -> Self {
        self.params.insert(
            "shadowcoef".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadowcoef_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowcoef".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_existcoef(mut self, val: f32) -> Self {
        self.params.insert(
            "existcoef".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_existcoef_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "existcoef".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uscale(mut self, val: f32) -> Self {
        self.params.insert(
            "uscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vscale(mut self, val: f32) -> Self {
        self.params.insert(
            "vscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uoff(mut self, val: f32) -> Self {
        self.params.insert(
            "uoff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_voff(mut self, val: f32) -> Self {
        self.params.insert(
            "voff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_voff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "voff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_amb(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "amb".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_amb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diff(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "diff".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_spec(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spec".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spec_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spec".to_string(),
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
    pub fn with_trans(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "trans".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_trans_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trans".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emit(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "emit".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_emit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_fmin(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "fmin".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_fmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fmax(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "fmax".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_fmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_spectype(mut self, val: &str) -> Self {
        self.params.insert(
            "spectype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_spectype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spectype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ambmap(mut self, val: &str) -> Self {
        self.params.insert(
            "ambmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ambmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ambmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ambchan(mut self, val: &str) -> Self {
        self.params.insert(
            "ambchan".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ambchan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ambchan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ambcomp(mut self, val: &str) -> Self {
        self.params.insert(
            "ambcomp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ambcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ambcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffmap(mut self, val: &str) -> Self {
        self.params.insert(
            "diffmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_diffmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffchan(mut self, val: &str) -> Self {
        self.params.insert(
            "diffchan".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_diffchan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffchan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_diffcomp(mut self, val: &str) -> Self {
        self.params.insert(
            "diffcomp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_diffcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "diffcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specmap(mut self, val: &str) -> Self {
        self.params.insert(
            "specmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_specmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specchan(mut self, val: &str) -> Self {
        self.params.insert(
            "specchan".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_specchan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specchan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_speccomp(mut self, val: &str) -> Self {
        self.params.insert(
            "speccomp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_speccomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speccomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflmap(mut self, val: &str) -> Self {
        self.params.insert(
            "reflmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reflmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflchan(mut self, val: &str) -> Self {
        self.params.insert(
            "reflchan".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reflchan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflchan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflcomp(mut self, val: &str) -> Self {
        self.params.insert(
            "reflcomp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reflcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transmap(mut self, val: &str) -> Self {
        self.params.insert(
            "transmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_transmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transchan(mut self, val: &str) -> Self {
        self.params.insert(
            "transchan".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_transchan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transchan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transcomp(mut self, val: &str) -> Self {
        self.params.insert(
            "transcomp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_transcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emitmap(mut self, val: &str) -> Self {
        self.params.insert(
            "emitmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_emitmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emitmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emitchan(mut self, val: &str) -> Self {
        self.params.insert(
            "emitchan".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_emitchan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emitchan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emitcomp(mut self, val: &str) -> Self {
        self.params.insert(
            "emitcomp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_emitcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emitcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_droughmap(mut self, val: &str) -> Self {
        self.params.insert(
            "droughmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_droughmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "droughmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_droughchan(mut self, val: &str) -> Self {
        self.params.insert(
            "droughchan".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_droughchan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "droughchan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_droughcomp(mut self, val: &str) -> Self {
        self.params.insert(
            "droughcomp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_droughcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "droughcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sroughmap(mut self, val: &str) -> Self {
        self.params.insert(
            "sroughmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sroughmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sroughmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sroughchan(mut self, val: &str) -> Self {
        self.params.insert(
            "sroughchan".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sroughchan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sroughchan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sroughcomp(mut self, val: &str) -> Self {
        self.params.insert(
            "sroughcomp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sroughcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sroughcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractmap(mut self, val: &str) -> Self {
        self.params.insert(
            "refractmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refractmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractchan(mut self, val: &str) -> Self {
        self.params.insert(
            "refractchan".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refractchan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractchan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refractcomp(mut self, val: &str) -> Self {
        self.params.insert(
            "refractcomp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refractcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refractcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aparamap(mut self, val: &str) -> Self {
        self.params.insert(
            "aparamap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aparamap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aparamap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aparachan(mut self, val: &str) -> Self {
        self.params.insert(
            "aparachan".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aparachan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aparachan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aparacomp(mut self, val: &str) -> Self {
        self.params.insert(
            "aparacomp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aparacomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aparacomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aperpmap(mut self, val: &str) -> Self {
        self.params.insert(
            "aperpmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aperpmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aperpmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aperpchan(mut self, val: &str) -> Self {
        self.params.insert(
            "aperpchan".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aperpchan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aperpchan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aperpcomp(mut self, val: &str) -> Self {
        self.params.insert(
            "aperpcomp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aperpcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aperpcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_arollmap(mut self, val: &str) -> Self {
        self.params.insert(
            "arollmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_arollmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "arollmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_arollchan(mut self, val: &str) -> Self {
        self.params.insert(
            "arollchan".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_arollchan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "arollchan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_arollcomp(mut self, val: &str) -> Self {
        self.params.insert(
            "arollcomp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_arollcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "arollcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowmap(mut self, val: &str) -> Self {
        self.params.insert(
            "shadowmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shadowmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowchan(mut self, val: &str) -> Self {
        self.params.insert(
            "shadowchan".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shadowchan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowchan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowcomp(mut self, val: &str) -> Self {
        self.params.insert(
            "shadowcomp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shadowcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_existmap(mut self, val: &str) -> Self {
        self.params.insert(
            "existmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_existmap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "existmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_existchan(mut self, val: &str) -> Self {
        self.params.insert(
            "existchan".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_existchan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "existchan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_existcomp(mut self, val: &str) -> Self {
        self.params.insert(
            "existcomp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_existcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "existcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter(mut self, val: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_interp(mut self, val: bool) -> Self {
        self.params.insert(
            "interp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_interp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVSupermat {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_supermat"
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
pub struct ShopVTracerGeometry {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVTracerGeometry {
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
    pub fn with_now(mut self, val: f32) -> Self {
        self.params.insert(
            "now".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_now_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "now".to_string(),
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
    pub fn with_activeradius(mut self, val: f32) -> Self {
        self.params.insert(
            "activeradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_activeradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activeradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dist(mut self, val: f32) -> Self {
        self.params.insert(
            "dist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lscale(mut self, val: f32) -> Self {
        self.params.insert(
            "lscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_pos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_dir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lclr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lclr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lclr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lclr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_hit(mut self, val: i32) -> Self {
        self.params
            .insert("hit".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_hit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVTracerGeometry {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_tracer_geometry"
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
pub struct ShopVTracerPclight {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVTracerPclight {
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
    pub fn with_now(mut self, val: f32) -> Self {
        self.params.insert(
            "now".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_now_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "now".to_string(),
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
    pub fn with_activeradius(mut self, val: f32) -> Self {
        self.params.insert(
            "activeradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_activeradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activeradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dist(mut self, val: f32) -> Self {
        self.params.insert(
            "dist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lscale(mut self, val: f32) -> Self {
        self.params.insert(
            "lscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_pos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_dir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lclr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lclr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lclr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lclr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_hit(mut self, val: i32) -> Self {
        self.params
            .insert("hit".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_hit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "pcsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pcsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pcfile(mut self, val: &str) -> Self {
        self.params.insert(
            "pcfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pcfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVTracerPclight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_tracer_pclight"
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
pub struct ShopVTracerPcphoton {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVTracerPcphoton {
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
    pub fn with_now(mut self, val: f32) -> Self {
        self.params.insert(
            "now".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_now_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "now".to_string(),
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
    pub fn with_activeradius(mut self, val: f32) -> Self {
        self.params.insert(
            "activeradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_activeradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activeradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dist(mut self, val: f32) -> Self {
        self.params.insert(
            "dist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lscale(mut self, val: f32) -> Self {
        self.params.insert(
            "lscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_pos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_dir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lclr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lclr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lclr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lclr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_hit(mut self, val: i32) -> Self {
        self.params
            .insert("hit".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_hit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pcsamples(mut self, val: i32) -> Self {
        self.params.insert(
            "pcsamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pcsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pcfile(mut self, val: &str) -> Self {
        self.params.insert(
            "pcfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pcfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pcfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVTracerPcphoton {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_tracer_pcphoton"
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
pub struct ShopVUniform {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVUniform {
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

    // --- Float3 parameters ---
    pub fn with_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "clr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVUniform {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_uniform"
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
pub enum ShopVUvlensUnwrapMethod {
    UvToSurface = 0,
    TraceClosestSurface = 1,
    UvMatch = 2,
}

#[derive(Debug, Clone)]
pub struct ShopVUvlens {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVUvlens {
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
    pub fn with_ray_bias(mut self, val: f32) -> Self {
        self.params.insert(
            "ray_bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ray_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ray_bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ray_maxdist(mut self, val: f32) -> Self {
        self.params.insert(
            "ray_maxdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ray_maxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ray_maxdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_unwrap_method(mut self, val: ShopVUvlensUnwrapMethod) -> Self {
        self.params.insert(
            "unwrap_method".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_unwrap_method_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unwrap_method".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flipnormal(mut self, val: i32) -> Self {
        self.params.insert(
            "flipnormal".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_flipnormal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flipnormal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVUvlens {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_uvlens"
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
pub struct ShopVVolumecloud {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVVolumecloud {
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
    pub fn with_clouddensity(mut self, val: f32) -> Self {
        self.params.insert(
            "clouddensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clouddensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clouddensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowscale(mut self, val: f32) -> Self {
        self.params.insert(
            "shadowscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadowscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowscale".to_string(),
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
    pub fn with_diff(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "diff".to_string(),
            crate::core::types::ParamValue::Float3(val),
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

    // --- Toggle parameters ---
    pub fn with_receiveshadows(mut self, val: bool) -> Self {
        self.params.insert(
            "receiveshadows".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_receiveshadows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "receiveshadows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVVolumecloud {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_volumecloud"
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
pub struct ShopVVolumefire {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVVolumefire {
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
    pub fn with_lowval(mut self, val: f32) -> Self {
        self.params.insert(
            "lowval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lowval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lowval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_midval(mut self, val: f32) -> Self {
        self.params.insert(
            "midval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_midval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "midval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_highval(mut self, val: f32) -> Self {
        self.params.insert(
            "highval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_highval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "highval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flamedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "flamedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_flamedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flamedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "shadowdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadowdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emission(mut self, val: f32) -> Self {
        self.params.insert(
            "emission".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_emission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_lowcd(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lowcd".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lowcd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lowcd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_midcd(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "midcd".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_midcd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "midcd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_highcd(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "highcd".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_highcd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "highcd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_flamemap(mut self, val: &str) -> Self {
        self.params.insert(
            "flamemap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_flamemap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flamemap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_hollowinterior(mut self, val: bool) -> Self {
        self.params.insert(
            "hollowinterior".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hollowinterior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hollowinterior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVVolumefire {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_volumefire"
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
pub struct ShopVVolumematte {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVVolumematte {
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
    pub fn with_clouddensity(mut self, val: f32) -> Self {
        self.params.insert(
            "clouddensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clouddensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clouddensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowdensityscale(mut self, val: f32) -> Self {
        self.params.insert(
            "shadowdensityscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shadowdensityscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shadowdensityscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_cd(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "Cd".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Cd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_receiveshadows(mut self, val: bool) -> Self {
        self.params.insert(
            "receiveshadows".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_receiveshadows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "receiveshadows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVVolumematte {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_volumematte"
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
pub enum ShopVVrlensLayout {
    /// Left/Right
    LeftRight = 0,
    /// Left Over/Right Under
    LeftOverRightUnder = 1,
    Left = 2,
    Right = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopVVrlensProjection {
    Latlong = 0,
    Perspective = 1,
    /// Cube Map - NVIDIA
    CubeMapMinusNvidia = 2,
    /// Cube Map - 3x2
    CubeMapMinus3x2 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopVVrlensMergemode {
    None = 0,
    Linear = 1,
    Smooth = 2,
}

#[derive(Debug, Clone)]
pub struct ShopVVrlens {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVVrlens {
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
    pub fn with_mergeangle(mut self, val: f32) -> Self {
        self.params.insert(
            "mergeAngle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mergeangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mergeAngle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_horizontalfov(mut self, val: f32) -> Self {
        self.params.insert(
            "horizontalFOV".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_horizontalfov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "horizontalFOV".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verticalfov(mut self, val: f32) -> Self {
        self.params.insert(
            "verticalFOV".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_verticalfov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verticalFOV".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_perspectivefov(mut self, val: f32) -> Self {
        self.params.insert(
            "perspectiveFOV".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_perspectivefov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "perspectiveFOV".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_perspectiveclipnear(mut self, val: f32) -> Self {
        self.params.insert(
            "perspectiveClipNear".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_perspectiveclipnear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "perspectiveClipNear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_perspectiveclipfar(mut self, val: f32) -> Self {
        self.params.insert(
            "perspectiveClipFar".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_perspectiveclipfar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "perspectiveClipFar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_perspectivedistort(mut self, val: f32) -> Self {
        self.params.insert(
            "perspectiveDistort".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_perspectivedistort_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "perspectiveDistort".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_perspectivedistortcubic(mut self, val: f32) -> Self {
        self.params.insert(
            "perspectiveDistortCubic".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_perspectivedistortcubic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "perspectiveDistortCubic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eyeseparation(mut self, val: f32) -> Self {
        self.params.insert(
            "eyeSeparation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eyeseparation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeSeparation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eyetoneckdistance(mut self, val: f32) -> Self {
        self.params.insert(
            "eyeToNeckDistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eyetoneckdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeToNeckDistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_layout(mut self, val: ShopVVrlensLayout) -> Self {
        self.params.insert(
            "layout".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_layout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "layout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_projection(mut self, val: ShopVVrlensProjection) -> Self {
        self.params.insert(
            "projection".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_projection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mergemode(mut self, val: ShopVVrlensMergemode) -> Self {
        self.params.insert(
            "mergeMode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_mergemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mergeMode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_swapleftright(mut self, val: bool) -> Self {
        self.params.insert(
            "swapLeftRight".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_swapleftright_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "swapLeftRight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preserveaspectratio(mut self, val: bool) -> Self {
        self.params.insert(
            "preserveAspectRatio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_preserveaspectratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preserveAspectRatio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usestereoeye(mut self, val: bool) -> Self {
        self.params.insert(
            "useStereoEye".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usestereoeye_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useStereoEye".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVVrlens {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_vrlens"
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
pub struct ShopVWindow {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVWindow {
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
    pub fn with_xoff(mut self, val: f32) -> Self {
        self.params.insert(
            "xoff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yoff(mut self, val: f32) -> Self {
        self.params.insert(
            "yoff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_yoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_panewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "panewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_panewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "panewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_paneheight(mut self, val: f32) -> Self {
        self.params.insert(
            "paneheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_paneheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "paneheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_framewidth(mut self, val: f32) -> Self {
        self.params.insert(
            "framewidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_framewidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "framewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fuzz(mut self, val: f32) -> Self {
        self.params.insert(
            "fuzz".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fuzz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fuzz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_light_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "light_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_light_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "light_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dark_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dark_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dark_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dark_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_xorder(mut self, val: i32) -> Self {
        self.params.insert(
            "xorder".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_xorder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xorder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yorder(mut self, val: i32) -> Self {
        self.params.insert(
            "yorder".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_yorder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yorder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVWindow {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_window"
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
pub struct ShopVZdepth {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVZdepth {
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
    pub fn with_near(mut self, val: f32) -> Self {
        self.params.insert(
            "near".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_near_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "near".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_far(mut self, val: f32) -> Self {
        self.params.insert(
            "far".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_far_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "far".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rolloff(mut self, val: f32) -> Self {
        self.params.insert(
            "rolloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rolloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rolloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_clr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "clr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_clr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVZdepth {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "v_zdepth"
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
pub struct ShopVmGeoAgent {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVmGeoAgent {
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
    pub fn with_lod(mut self, val: f32) -> Self {
        self.params.insert(
            "lod".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablelod(mut self, val: bool) -> Self {
        self.params.insert(
            "enablelod".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablelod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablelod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optimizeexactinstances(mut self, val: bool) -> Self {
        self.params.insert(
            "optimizeexactinstances".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_optimizeexactinstances_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "optimizeexactinstances".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVmGeoAgent {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vm_geo_agent"
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
pub struct ShopVmGeoAlembic {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVmGeoAlembic {
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
    pub fn with_frame(mut self, val: f32) -> Self {
        self.params.insert(
            "frame".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frame_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frame".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fps(mut self, val: f32) -> Self {
        self.params.insert(
            "fps".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_filename(mut self, val: &str) -> Self {
        self.params.insert(
            "filename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_objectpath(mut self, val: &str) -> Self {
        self.params.insert(
            "objectpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_objectpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objectpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_objectpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "objectpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_objectpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objectpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_userpropertymap(mut self, val: &str) -> Self {
        self.params.insert(
            "userpropertymap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_userpropertymap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "userpropertymap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribfile(mut self, val: &str) -> Self {
        self.params.insert(
            "attribfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "pointattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vertexattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "vertexattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vertexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vertexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "uniformattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uniformattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_detailattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "detailattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_detailattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "detailattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useobjectgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "useobjectgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useobjectgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useobjectgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nonalembic(mut self, val: bool) -> Self {
        self.params.insert(
            "nonalembic".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_nonalembic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nonalembic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVmGeoAlembic {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vm_geo_alembic"
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
pub struct ShopVmGeoEngine {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVmGeoEngine {
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

    // --- String parameters ---
    pub fn with_opname(mut self, val: &str) -> Self {
        self.params.insert(
            "opname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_otllist(mut self, val: &str) -> Self {
        self.params.insert(
            "otllist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_otllist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "otllist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_objecthda(mut self, val: bool) -> Self {
        self.params.insert(
            "objecthda".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_objecthda_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objecthda".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doorient(mut self, val: bool) -> Self {
        self.params.insert(
            "doorient".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defergeo(mut self, val: bool) -> Self {
        self.params.insert(
            "defergeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_defergeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defergeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_requirelod(mut self, val: bool) -> Self {
        self.params.insert(
            "requirelod".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_requirelod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requirelod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVmGeoEngine {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vm_geo_engine"
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
pub struct ShopVmGeoEnginecurvegen {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVmGeoEnginecurvegen {
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

    // --- String parameters ---
    pub fn with_opname(mut self, val: &str) -> Self {
        self.params.insert(
            "opname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_otllist(mut self, val: &str) -> Self {
        self.params.insert(
            "otllist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_otllist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "otllist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_objecthda(mut self, val: bool) -> Self {
        self.params.insert(
            "objecthda".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_objecthda_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objecthda".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doorient(mut self, val: bool) -> Self {
        self.params.insert(
            "doorient".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defergeo(mut self, val: bool) -> Self {
        self.params.insert(
            "defergeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_defergeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defergeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_requirelod(mut self, val: bool) -> Self {
        self.params.insert(
            "requirelod".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_requirelod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requirelod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVmGeoEnginecurvegen {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vm_geo_enginecurvegen"
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
pub struct ShopVmGeoEnginepointgen {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVmGeoEnginepointgen {
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

    // --- String parameters ---
    pub fn with_opname(mut self, val: &str) -> Self {
        self.params.insert(
            "opname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_otllist(mut self, val: &str) -> Self {
        self.params.insert(
            "otllist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_otllist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "otllist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_objecthda(mut self, val: bool) -> Self {
        self.params.insert(
            "objecthda".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_objecthda_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objecthda".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doorient(mut self, val: bool) -> Self {
        self.params.insert(
            "doorient".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defergeo(mut self, val: bool) -> Self {
        self.params.insert(
            "defergeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_defergeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defergeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_requirelod(mut self, val: bool) -> Self {
        self.params.insert(
            "requirelod".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_requirelod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requirelod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVmGeoEnginepointgen {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vm_geo_enginepointgen"
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
pub struct ShopVmGeoFile {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVmGeoFile {
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
    pub fn with_frameoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "frameoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frameoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frameoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shutter(mut self, val: f32) -> Self {
        self.params.insert(
            "shutter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shutter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shutter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_file(mut self, val: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "file".to_string(),
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
    pub fn with_blurfile(mut self, val: &str) -> Self {
        self.params.insert(
            "blurfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blurfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blurfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matfile(mut self, val: &str) -> Self {
        self.params.insert(
            "matfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_velocityblur(mut self, val: bool) -> Self {
        self.params.insert(
            "velocityblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocityblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocityblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharegeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "sharegeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sharegeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharegeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVmGeoFile {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vm_geo_file"
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
pub struct ShopVmGeoFur {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVmGeoFur {
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
    pub fn with_expandbounds(mut self, val: f32) -> Self {
        self.params.insert(
            "expandbounds".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_expandbounds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expandbounds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_length(mut self, val: f32) -> Self {
        self.params.insert(
            "length".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_length_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "length".to_string(),
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
    pub fn with_display(mut self, val: f32) -> Self {
        self.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Float(val),
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
    pub fn with_guideradius(mut self, val: f32) -> Self {
        self.params.insert(
            "guideradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guideradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clumpradius(mut self, val: f32) -> Self {
        self.params.insert(
            "clumpradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clumpradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clumpradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partingradius(mut self, val: f32) -> Self {
        self.params.insert(
            "partingradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_partingradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partingradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_displayboundsmin(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "displayboundsmin".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_displayboundsmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayboundsmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displayboundsmax(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "displayboundsmax".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_displayboundsmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayboundsmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_segs(mut self, val: i32) -> Self {
        self.params
            .insert("segs".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_segs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "segs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed(mut self, val: i32) -> Self {
        self.params
            .insert("seed".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_skin(mut self, val: &str) -> Self {
        self.params.insert(
            "skin".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guides(mut self, val: &str) -> Self {
        self.params.insert(
            "guides".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guides_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guides".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clumps(mut self, val: &str) -> Self {
        self.params.insert(
            "clumps".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clumps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clumps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partinglines(mut self, val: &str) -> Self {
        self.params.insert(
            "partinglines".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_partinglines_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partinglines".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_skinshader(mut self, val: &str) -> Self {
        self.params.insert(
            "skinshader".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skinshader_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skinshader".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinattribclass(mut self, val: &str) -> Self {
        self.params.insert(
            "skinattribclass".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skinattribclass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skinattribclass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skintextureattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "skintextureattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skintextureattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skintextureattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideshader(mut self, val: &str) -> Self {
        self.params.insert(
            "guideshader".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guideshader_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideshader".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideattribclass(mut self, val: &str) -> Self {
        self.params.insert(
            "guideattribclass".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guideattribclass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideattribclass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidetextureattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "guidetextureattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guidetextureattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidetextureattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_limittodisplaybounds(mut self, val: bool) -> Self {
        self.params.insert(
            "limittodisplaybounds".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limittodisplaybounds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limittodisplaybounds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useclosestclump(mut self, val: bool) -> Self {
        self.params.insert(
            "useclosestclump".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useclosestclump_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useclosestclump".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removeunclumpedhairs(mut self, val: bool) -> Self {
        self.params.insert(
            "removeunclumpedhairs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removeunclumpedhairs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "removeunclumpedhairs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removeunguidedhairs(mut self, val: bool) -> Self {
        self.params.insert(
            "removeunguidedhairs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removeunguidedhairs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "removeunguidedhairs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setid(mut self, val: bool) -> Self {
        self.params.insert(
            "setid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVmGeoFur {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vm_geo_fur"
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
pub struct ShopVmGeoImage3d {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVmGeoImage3d {
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
    pub fn with_lod(mut self, val: f32) -> Self {
        self.params.insert(
            "lod".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lod".to_string(),
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

    // --- String parameters ---
    pub fn with_file(mut self, val: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_channel(mut self, val: &str) -> Self {
        self.params.insert(
            "channel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_channel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "channel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter(mut self, val: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVmGeoImage3d {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vm_geo_image3d"
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
pub struct ShopVmGeoImage3dvolume {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVmGeoImage3dvolume {
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

    // --- String parameters ---
    pub fn with_file(mut self, val: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVmGeoImage3dvolume {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vm_geo_image3dvolume"
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
pub struct ShopVmGeoProgram {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVmGeoProgram {
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
    pub fn with_shutter(mut self, val: f32) -> Self {
        self.params.insert(
            "shutter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shutter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shutter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_program(mut self, val: &str) -> Self {
        self.params.insert(
            "program".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_program_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "program".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blurprogram(mut self, val: &str) -> Self {
        self.params.insert(
            "blurprogram".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blurprogram_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blurprogram".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVmGeoProgram {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vm_geo_program"
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
pub struct ShopVmGeoPtinstance {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVmGeoPtinstance {
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

    // --- String parameters ---
    pub fn with_pointpath(mut self, val: &str) -> Self {
        self.params.insert(
            "pointpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instancepath(mut self, val: &str) -> Self {
        self.params.insert(
            "instancepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_instancepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instancepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ptmotionblur(mut self, val: &str) -> Self {
        self.params.insert(
            "ptmotionblur".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ptmotionblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ptmotionblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cvexscript(mut self, val: &str) -> Self {
        self.params.insert(
            "cvexscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cvexscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cvexscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cvexattributemask(mut self, val: &str) -> Self {
        self.params.insert(
            "cvexattributemask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cvexattributemask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cvexattributemask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_instancexform(mut self, val: bool) -> Self {
        self.params.insert(
            "instancexform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_instancexform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instancexform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderboxes(mut self, val: bool) -> Self {
        self.params.insert(
            "renderboxes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_renderboxes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderboxes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVmGeoPtinstance {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vm_geo_ptinstance"
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
pub struct ShopVmGeoPtreplicate {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVmGeoPtreplicate {
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

    // --- Int parameters ---
    pub fn with_multiplier(mut self, val: i32) -> Self {
        self.params.insert(
            "multiplier".to_string(),
            crate::core::types::ParamValue::Int(val),
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
    pub fn with_unpack(mut self, val: i32) -> Self {
        self.params.insert(
            "unpack".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_unpack_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unpack".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pointpath(mut self, val: &str) -> Self {
        self.params.insert(
            "pointpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributemask(mut self, val: &str) -> Self {
        self.params.insert(
            "attributemask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attributemask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributemask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_motionblur(mut self, val: &str) -> Self {
        self.params.insert(
            "motionblur".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_motionblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motionblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cvexscript(mut self, val: &str) -> Self {
        self.params.insert(
            "cvexscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cvexscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cvexscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVmGeoPtreplicate {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vm_geo_ptreplicate"
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
pub struct ShopVmGeoSprite {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVmGeoSprite {
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

    // --- String parameters ---
    pub fn with_object(mut self, val: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity(mut self, val: &str) -> Self {
        self.params.insert(
            "velocity".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_attribute(mut self, val: &str) -> Self {
        self.params.insert(
            "attribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVmGeoSprite {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vm_geo_sprite"
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
pub struct ShopVmGeoVexvolume {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVmGeoVexvolume {
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

    // --- Int parameters ---
    pub fn with_divisions(mut self, val: i32) -> Self {
        self.params.insert(
            "divisions".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_divisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_shader(mut self, val: &str) -> Self {
        self.params.insert(
            "shader".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shader_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shader".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVmGeoVexvolume {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vm_geo_vexvolume"
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
pub struct ShopVopcvex {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVopcvex {
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

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVopcvex {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopcvex"
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
pub struct ShopVopcvextype {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVopcvextype {
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

    // --- String parameters ---
    pub fn with_englishname(mut self, val: &str) -> Self {
        self.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_englishname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_tabmenuflag(mut self, val: bool) -> Self {
        self.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tabmenuflag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVopcvextype {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopcvextype"
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
pub struct ShopVopdisplace {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVopdisplace {
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

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVopdisplace {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopdisplace"
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
pub struct ShopVopdisplacetype {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVopdisplacetype {
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

    // --- String parameters ---
    pub fn with_englishname(mut self, val: &str) -> Self {
        self.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_englishname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_tabmenuflag(mut self, val: bool) -> Self {
        self.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tabmenuflag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVopdisplacetype {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopdisplacetype"
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
pub struct ShopVopfog {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVopfog {
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

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVopfog {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopfog"
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
pub struct ShopVopfogtype {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVopfogtype {
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

    // --- String parameters ---
    pub fn with_englishname(mut self, val: &str) -> Self {
        self.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_englishname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_tabmenuflag(mut self, val: bool) -> Self {
        self.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tabmenuflag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVopfogtype {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopfogtype"
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
pub struct ShopVopimage3d {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVopimage3d {
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

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVopimage3d {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopimage3d"
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
pub struct ShopVopimage3dtype {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVopimage3dtype {
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

    // --- String parameters ---
    pub fn with_englishname(mut self, val: &str) -> Self {
        self.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_englishname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_tabmenuflag(mut self, val: bool) -> Self {
        self.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tabmenuflag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVopimage3dtype {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopimage3dtype"
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
pub struct ShopVoplight {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVoplight {
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

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVoplight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "voplight"
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
pub struct ShopVoplighttype {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVoplighttype {
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

    // --- String parameters ---
    pub fn with_englishname(mut self, val: &str) -> Self {
        self.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_englishname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_tabmenuflag(mut self, val: bool) -> Self {
        self.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tabmenuflag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVoplighttype {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "voplighttype"
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
pub struct ShopVopmaterial {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVopmaterial {
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

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVopmaterial {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopmaterial"
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
pub struct ShopVopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVopnet {
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
}

impl crate::core::types::HoudiniNode for ShopVopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopnet"
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
pub struct ShopVopphoton {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVopphoton {
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

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVopphoton {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopphoton"
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
pub struct ShopVopshadow {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVopshadow {
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

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVopshadow {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopshadow"
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
pub struct ShopVopshadowtype {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVopshadowtype {
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

    // --- String parameters ---
    pub fn with_englishname(mut self, val: &str) -> Self {
        self.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_englishname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_tabmenuflag(mut self, val: bool) -> Self {
        self.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tabmenuflag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVopshadowtype {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopshadowtype"
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
pub struct ShopVopsurface {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVopsurface {
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

    // --- Button parameters ---
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
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

    // --- Float3 parameters ---
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

    // --- String parameters ---
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
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
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
}

impl crate::core::types::HoudiniNode for ShopVopsurface {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopsurface"
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
pub struct ShopVopsurfacetype {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopVopsurfacetype {
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

    // --- String parameters ---
    pub fn with_englishname(mut self, val: &str) -> Self {
        self.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_englishname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "englishname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_tabmenuflag(mut self, val: bool) -> Self {
        self.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tabmenuflag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tabmenuflag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopVopsurfacetype {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopsurfacetype"
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
