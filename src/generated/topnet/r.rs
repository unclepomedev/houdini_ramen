#[derive(Debug, Clone)]
pub struct TopnetRemotegraph {
    pub base: crate::core::types::NodeBase,
}

impl TopnetRemotegraph {
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
    pub fn trigger_connectbutton(mut self) -> Self {
        self.base.params.insert(
            "connectbutton".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_clienttype(mut self, val: i32) -> Self {
        self.base.params.insert(
            "clienttype".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_clienttype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clienttype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remotegraph(mut self, val: i32) -> Self {
        self.base.params.insert(
            "remotegraph".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_remotegraph_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remotegraph".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_host(mut self, val: &str) -> Self {
        self.base.params.insert(
            "host".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_host_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "host".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopnetRemotegraph {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "remotegraph"
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
