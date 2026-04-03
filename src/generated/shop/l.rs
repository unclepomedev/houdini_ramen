#[derive(Debug, Clone)]
pub struct ShopLopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ShopLopnet {
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



    // --- Int parameters ---
    pub fn with_modifiedprimcounttostartnewlayer(mut self, val: i32) -> Self {
        self.params.insert("modifiedprimcounttostartnewlayer".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_modifiedprimcounttostartnewlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("modifiedprimcounttostartnewlayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_expansioneffect(mut self, val: &str) -> Self {
        self.params.insert("expansioneffect".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_expansioneffect_expr(mut self, expr: &str) -> Self {
        self.params.insert("expansioneffect".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pinnedprims(mut self, val: &str) -> Self {
        self.params.insert("pinnedprims".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pinnedprims_expr(mut self, expr: &str) -> Self {
        self.params.insert("pinnedprims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_resolvercontextassetpath(mut self, val: &str) -> Self {
        self.params.insert("resolvercontextassetpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_resolvercontextassetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("resolvercontextassetpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_resolvercontextstringurlprefix_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("resolvercontextstringurlprefix{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_resolvercontextstringurlprefix_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("resolvercontextstringurlprefix{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_resolvercontextstringvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("resolvercontextstringvalue{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_resolvercontextstringvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("resolvercontextstringvalue{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_variantselectionset_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("variantselectionset{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_variantselectionset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("variantselectionset{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_variantselectionvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("variantselectionvalue{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_variantselectionvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("variantselectionvalue{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_insertionpointdescriptor(mut self, val: &str) -> Self {
        self.params.insert("insertionpointdescriptor".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_insertionpointdescriptor_expr(mut self, expr: &str) -> Self {
        self.params.insert("insertionpointdescriptor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rendergallerysource(mut self, val: &str) -> Self {
        self.params.insert("rendergallerysource".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_rendergallerysource_expr(mut self, expr: &str) -> Self {
        self.params.insert("rendergallerysource".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_resolvercontextstringenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("resolvercontextstringenable{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_resolvercontextstringenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("resolvercontextstringenable{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_variantselectionenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("variantselectionenable{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_variantselectionenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("variantselectionenable{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ShopLopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "lopnet"
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
