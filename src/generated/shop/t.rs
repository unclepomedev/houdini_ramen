#[derive(Debug, Clone)]
pub struct ShopToonoutlineshader {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopToonoutlineshader {
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
    pub fn with_outlinescenethickness(mut self, val: f32) -> Self {
        self.params.insert(
            "outlinescenethickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_outlinescenethickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlinescenethickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outlineimagethickness(mut self, val: f32) -> Self {
        self.params.insert(
            "outlineimagethickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_outlineimagethickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlineimagethickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imagethicknessblend(mut self, val: f32) -> Self {
        self.params.insert(
            "imagethicknessblend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_imagethicknessblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imagethicknessblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spillin(mut self, val: f32) -> Self {
        self.params.insert(
            "spillin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spillin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spillin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxobjdepth(mut self, val: f32) -> Self {
        self.params.insert(
            "maxobjdepth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxobjdepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxobjdepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_outlinecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "outlinecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_outlinecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outlinecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopToonoutlineshader {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "toonoutlineshader"
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
pub struct ShopToonshading {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopToonshading {
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
    pub fn with_difframp_min(mut self, val: f32) -> Self {
        self.params.insert(
            "difframp_min".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_difframp_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "difframp_min".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_difframp_max(mut self, val: f32) -> Self {
        self.params.insert(
            "difframp_max".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_difframp_max_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "difframp_max".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_difftex_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "difftex_blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_difftex_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "difftex_blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossramp_min(mut self, val: f32) -> Self {
        self.params.insert(
            "glossramp_min".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glossramp_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossramp_min".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glossramp_max(mut self, val: f32) -> Self {
        self.params.insert(
            "glossramp_max".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glossramp_max_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glossramp_max".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatramp_min(mut self, val: f32) -> Self {
        self.params.insert(
            "coatramp_min".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coatramp_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatramp_min".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coatramp_max(mut self, val: f32) -> Self {
        self.params.insert(
            "coatramp_max".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coatramp_max_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coatramp_max".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_albedo(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "albedo".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_albedo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "albedo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directdiffuse(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "directdiffuse".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_directdiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directdiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indirectdiffuse(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "indirectdiffuse".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_indirectdiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indirectdiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sss(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sss".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sss_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sss".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflecteddirectdiffuse(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "reflecteddirectdiffuse".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_reflecteddirectdiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflecteddirectdiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflectedalbedodiffuse(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "reflectedalbedodiffuse".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_reflectedalbedodiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflectedalbedodiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflectedglossy(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "reflectedglossy".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_reflectedglossy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflectedglossy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflectedcoat(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "reflectedcoat".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_reflectedcoat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflectedcoat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transmittedglossydiffuse(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "transmittedglossydiffuse".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_transmittedglossydiffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transmittedglossydiffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toon_diffuse(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "toon_diffuse".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_toon_diffuse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "toon_diffuse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toon_glossy(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "toon_glossy".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_toon_glossy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "toon_glossy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toon_coat(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "toon_coat".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_toon_coat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "toon_coat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_outline(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "outline".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_outline_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_c(mut self, val: [f32; 4]) -> Self {
        self.params
            .insert("C".to_string(), crate::core::types::ParamValue::Float4(val));
        self
    }
    pub fn with_c_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "C".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toon_outline(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "toon_outline".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_toon_outline_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "toon_outline".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_outline_glossy(mut self, val: i32) -> Self {
        self.params.insert(
            "outline_glossy".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_outline_glossy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outline_glossy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apply_albedo(mut self, val: i32) -> Self {
        self.params.insert(
            "apply_albedo".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_apply_albedo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "apply_albedo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopToonshading {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "toonshading"
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
pub enum ShopTopnetCheckpointformat {
    /// Python (Deprecated)
    PythonDeprecated = 0,
    Json = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopTopnetCheckpointload {
    Never = 0,
    OnSceneLoad = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopTopnetRegenerationtype {
    UpdateWorkItemsAndInvalidateCaches = 0,
    UpdateWorkItemsOnly = 1,
    IgnoreChanges = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopTopnetEvaluationtime {
    NetworkCookTime = 0,
    GlobalStartTime = 1,
    Custom = 2,
}

#[derive(Debug, Clone)]
pub struct ShopTopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopTopnet {
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
    pub fn trigger_generatestatic(mut self) -> Self {
        self.params.insert(
            "generatestatic".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_cookbutton(mut self) -> Self {
        self.params.insert(
            "cookbutton".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_dirtybutton(mut self) -> Self {
        self.params.insert(
            "dirtybutton".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_cancelbutton(mut self) -> Self {
        self.params.insert(
            "cancelbutton".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_savetaskgraph(mut self) -> Self {
        self.params.insert(
            "savetaskgraph".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_loadtaskgraph(mut self) -> Self {
        self.params.insert(
            "loadtaskgraph".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_loadcheckpoint(mut self) -> Self {
        self.params.insert(
            "loadcheckpoint".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Int parameters ---
    pub fn with_taskgraphsaverate(mut self, val: i32) -> Self {
        self.params.insert(
            "taskgraphsaverate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_taskgraphsaverate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "taskgraphsaverate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointrate(mut self, val: i32) -> Self {
        self.params.insert(
            "checkpointrate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_checkpointrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customtime(mut self, val: i32) -> Self {
        self.params.insert(
            "customtime".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_customtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_checkpointformat(mut self, val: ShopTopnetCheckpointformat) -> Self {
        self.params.insert(
            "checkpointformat".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_checkpointformat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointformat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointload(mut self, val: ShopTopnetCheckpointload) -> Self {
        self.params.insert(
            "checkpointload".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_checkpointload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_regenerationtype(mut self, val: ShopTopnetRegenerationtype) -> Self {
        self.params.insert(
            "regenerationtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_regenerationtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "regenerationtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_evaluationtime(mut self, val: ShopTopnetEvaluationtime) -> Self {
        self.params.insert(
            "evaluationtime".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_evaluationtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "evaluationtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_taskgraphfile(mut self, val: &str) -> Self {
        self.params.insert(
            "taskgraphfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_taskgraphfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "taskgraphfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointfile(mut self, val: &str) -> Self {
        self.params.insert(
            "checkpointfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_checkpointfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultlabel(mut self, val: &str) -> Self {
        self.params.insert(
            "defaultlabel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_defaultlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_taskgraphautosave(mut self, val: bool) -> Self {
        self.params.insert(
            "taskgraphautosave".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_taskgraphautosave_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "taskgraphautosave".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointenabled(mut self, val: bool) -> Self {
        self.params.insert(
            "checkpointenabled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_checkpointenabled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointenabled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savegraphattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "savegraphattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savegraphattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savegraphattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedefaultlabel(mut self, val: bool) -> Self {
        self.params.insert(
            "usedefaultlabel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedefaultlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedefaultlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savescenefile(mut self, val: bool) -> Self {
        self.params.insert(
            "savescenefile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savescenefile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savescenefile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ShopTopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "topnet"
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
pub struct ShopTranslucencyEval {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopTranslucencyEval {
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
    pub fn with_sigma(mut self, val: f32) -> Self {
        self.params.insert(
            "sigma".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sigma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sigma".to_string(),
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

impl crate::core::types::HoudiniNode for ShopTranslucencyEval {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "translucency_eval"
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
pub struct ShopTranslucencySample {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopTranslucencySample {
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

impl crate::core::types::HoudiniNode for ShopTranslucencySample {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "translucency_sample"
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
