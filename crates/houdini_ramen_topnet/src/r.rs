#[derive(Debug, Clone)]
pub struct TopnetRemotegraph {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopnetRemotegraph {
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
    pub fn trigger_connectbutton(mut self) -> Self {
        self.params.insert(
            "connectbutton".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_clienttype(mut self, val: i32) -> Self {
        self.params.insert(
            "clienttype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_clienttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clienttype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remotegraph(mut self, val: i32) -> Self {
        self.params.insert(
            "remotegraph".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_remotegraph_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remotegraph".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_host(mut self, val: &str) -> Self {
        self.params.insert(
            "host".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_host_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "host".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopnetRemotegraph {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "remotegraph"
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
