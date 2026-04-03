#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopLinktosourceobjectSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopLinktosourceobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopLinktosourceobject {
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

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input 0"
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 0" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert("activation".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert("activation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_sharedata(mut self, val: DopLinktosourceobjectSharedata) -> Self {
        self.params.insert("sharedata".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert("sharedata".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_objpath(mut self, val: &str) -> Self {
        self.params.insert("objpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_objpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("objpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for DopLinktosourceobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "linktosourceobject"
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
pub struct DopLopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopLopnet {
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

impl crate::core::types::HoudiniNode for DopLopnet {
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
