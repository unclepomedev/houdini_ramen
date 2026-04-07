#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopnetCopnetBorder {
    Constant = 0,
    Clamp = 1,
    Mirror = 2,
    Wrap = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopnetCopnetPrecision {
    /// 16-bit
    N16MinusBit = 0,
    /// 32-bit
    N32MinusBit = 1,
}

#[derive(Debug, Clone)]
pub struct CopnetCopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopnetCopnet {
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
    pub fn with_pixelscale(mut self, val: f32) -> Self {
        self.params.insert(
            "pixelscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pixelscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pixelscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_udim(mut self, val: i32) -> Self {
        self.params
            .insert("udim".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_udim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "udim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vistile(mut self, val: i32) -> Self {
        self.params.insert(
            "vistile".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vistile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vistile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_res(mut self, val: [i32; 2]) -> Self {
        self.params
            .insert("res".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_res_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_resmenu(mut self, val: i32) -> Self {
        self.params.insert(
            "resmenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_resmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_border(mut self, val: CopnetCopnetBorder) -> Self {
        self.params.insert(
            "border".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_border_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "border".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_precision(mut self, val: CopnetCopnetPrecision) -> Self {
        self.params.insert(
            "precision".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_precision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_setres(mut self, val: bool) -> Self {
        self.params.insert(
            "setres".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setpixelscale(mut self, val: bool) -> Self {
        self.params.insert(
            "setpixelscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setpixelscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setpixelscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setborder(mut self, val: bool) -> Self {
        self.params.insert(
            "setborder".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setborder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setborder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setprecision(mut self, val: bool) -> Self {
        self.params.insert(
            "setprecision".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setudim(mut self, val: bool) -> Self {
        self.params.insert(
            "setudim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setudim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setudim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setvistile(mut self, val: bool) -> Self {
        self.params.insert(
            "setvistile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setvistile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setvistile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopnetCopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "copnet"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub struct CopnetCopnetFilterlist {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopnetCopnetFilterlist {
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
    pub fn with_primaryscope(mut self, val: &str) -> Self {
        self.params.insert(
            "primaryscope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primaryscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primaryscope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthscope(mut self, val: &str) -> Self {
        self.params.insert(
            "depthscope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_depthscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthscope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tabmenufilter(mut self, val: &str) -> Self {
        self.params.insert(
            "tabmenufilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tabmenufilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tabmenufilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for CopnetCopnetFilterlist {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "copnet_filterlist"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
