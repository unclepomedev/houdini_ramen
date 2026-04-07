#[derive(Debug, Clone)]
pub struct VopWatersurf {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopWatersurf {
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

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: ""
    pub fn set_input_input_11<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "" and specifies the output index of the target node.
    pub fn set_input_input_11_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    /// Connects to input 12: ""
    pub fn set_input_input_12<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(12, (target.get_id(), 0));
        self
    }

    /// Connects to input 12: "" and specifies the output index of the target node.
    pub fn set_input_input_12_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(12, (target.get_id(), output_index));
        self
    }

    /// Connects to input 13: ""
    pub fn set_input_input_13<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(13, (target.get_id(), 0));
        self
    }

    /// Connects to input 13: "" and specifies the output index of the target node.
    pub fn set_input_input_13_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(13, (target.get_id(), output_index));
        self
    }

    /// Connects to input 14: ""
    pub fn set_input_input_14<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(14, (target.get_id(), 0));
        self
    }

    /// Connects to input 14: "" and specifies the output index of the target node.
    pub fn set_input_input_14_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(14, (target.get_id(), output_index));
        self
    }

    /// Connects to input 15: ""
    pub fn set_input_input_15<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(15, (target.get_id(), 0));
        self
    }

    /// Connects to input 15: "" and specifies the output index of the target node.
    pub fn set_input_input_15_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(15, (target.get_id(), output_index));
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
    pub fn with_waterclarity(mut self, val: f32) -> Self {
        self.params.insert(
            "waterclarity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_waterclarity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "waterclarity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_waterdepth(mut self, val: f32) -> Self {
        self.params.insert(
            "waterdepth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_waterdepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "waterdepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflscale(mut self, val: f32) -> Self {
        self.params.insert(
            "reflscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthbright(mut self, val: f32) -> Self {
        self.params.insert(
            "depthbright".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_depthbright_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthbright".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weedamount(mut self, val: f32) -> Self {
        self.params.insert(
            "weedamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weedamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weedamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_shallowcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shallowcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shallowcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shallowcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_midcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "midcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_midcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "midcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deepcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "deepcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_deepcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deepcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dirtycolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dirtycolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dirtycolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dirtycolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weedcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "weedcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_weedcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weedcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_speccolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "speccolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_speccolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speccolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthfreq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "depthfreq".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_depthfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weedfreq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "weedfreq".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_weedfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weedfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopWatersurf {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "watersurf"
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
pub struct VopWaves {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopWaves {
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

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: ""
    pub fn set_input_input_11<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "" and specifies the output index of the target node.
    pub fn set_input_input_11_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_wfreq(mut self, val: f32) -> Self {
        self.params.insert(
            "wfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wspeed(mut self, val: f32) -> Self {
        self.params.insert(
            "wspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wrandom(mut self, val: f32) -> Self {
        self.params.insert(
            "wrandom".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wrandom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wrandom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_camount(mut self, val: f32) -> Self {
        self.params.insert(
            "camount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_camount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

    // --- Float3 parameters ---
    pub fn with_wup(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "wup".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_wup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wdir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "wdir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_wdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cfreq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cfreq".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cspeed(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cspeed".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopWaves {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "waves"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopWavevectorPlane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone)]
pub struct VopWavevector {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopWavevector {
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

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_ix(mut self, val: i32) -> Self {
        self.params
            .insert("ix".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_ix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_iy(mut self, val: i32) -> Self {
        self.params
            .insert("iy".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_iy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_iz(mut self, val: i32) -> Self {
        self.params
            .insert("iz".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_iz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resx(mut self, val: i32) -> Self {
        self.params
            .insert("resx".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_resx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resy(mut self, val: i32) -> Self {
        self.params
            .insert("resy".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_resy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resz(mut self, val: i32) -> Self {
        self.params
            .insert("resz".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_resz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_plane(mut self, val: VopWavevectorPlane) -> Self {
        self.params.insert(
            "plane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_plane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "plane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_twod(mut self, val: bool) -> Self {
        self.params.insert(
            "twod".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_twod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "twod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopWavevector {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "wavevector"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopWhileCondition {
    /// True ( != 0 )
    True0 = 0,
    /// False ( == 0 )
    False0 = 1,
}

#[derive(Debug, Clone)]
pub struct VopWhile {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopWhile {
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

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Menu parameters ---
    pub fn with_condition(mut self, val: VopWhileCondition) -> Self {
        self.params.insert(
            "condition".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_condition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopWhile {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "while"
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
pub struct VopWirePattern {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopWirePattern {
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

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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
    pub fn with_ufreq(mut self, val: f32) -> Self {
        self.params.insert(
            "Ufreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ufreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Ufreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vfreq(mut self, val: f32) -> Self {
        self.params.insert(
            "Vfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "Vfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: f32) -> Self {
        self.params
            .insert("s".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_t(mut self, val: f32) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopWirePattern {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "wire_pattern"
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
pub struct VopWireframe {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopWireframe {
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

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_sfreq(mut self, val: f32) -> Self {
        self.params.insert(
            "sfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tfreq(mut self, val: f32) -> Self {
        self.params.insert(
            "tfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "wwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_wcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "wcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_wcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopWireframe {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "wireframe"
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
pub struct VopWood {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopWood {
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

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: ""
    pub fn set_input_input_11<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "" and specifies the output index of the target node.
    pub fn set_input_input_11_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    /// Connects to input 12: ""
    pub fn set_input_input_12<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(12, (target.get_id(), 0));
        self
    }

    /// Connects to input 12: "" and specifies the output index of the target node.
    pub fn set_input_input_12_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(12, (target.get_id(), output_index));
        self
    }

    /// Connects to input 13: ""
    pub fn set_input_input_13<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(13, (target.get_id(), 0));
        self
    }

    /// Connects to input 13: "" and specifies the output index of the target node.
    pub fn set_input_input_13_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(13, (target.get_id(), output_index));
        self
    }

    /// Connects to input 14: ""
    pub fn set_input_input_14<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(14, (target.get_id(), 0));
        self
    }

    /// Connects to input 14: "" and specifies the output index of the target node.
    pub fn set_input_input_14_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(14, (target.get_id(), output_index));
        self
    }

    /// Connects to input 15: ""
    pub fn set_input_input_15<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(15, (target.get_id(), 0));
        self
    }

    /// Connects to input 15: "" and specifies the output index of the target node.
    pub fn set_input_input_15_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(15, (target.get_id(), output_index));
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
    pub fn with_vsfreq(mut self, val: f32) -> Self {
        self.params.insert(
            "vsfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vsfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vsfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vtfreq(mut self, val: f32) -> Self {
        self.params.insert(
            "vtfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vtfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vtfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vatten(mut self, val: f32) -> Self {
        self.params.insert(
            "vatten".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vatten_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vatten".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_namp(mut self, val: f32) -> Self {
        self.params.insert(
            "namp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_namp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "namp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gdepth(mut self, val: f32) -> Self {
        self.params.insert(
            "gdepth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gdepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gdepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_basecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "basecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_basecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_veincolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "veincolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_veincolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "veincolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_speccolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "speccolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_speccolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speccolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nfreq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "nfreq".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_nfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "noffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_noffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopWood {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "wood"
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
pub struct VopWoodplank {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopWoodplank {
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

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: ""
    pub fn set_input_input_11<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "" and specifies the output index of the target node.
    pub fn set_input_input_11_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    /// Connects to input 12: ""
    pub fn set_input_input_12<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(12, (target.get_id(), 0));
        self
    }

    /// Connects to input 12: "" and specifies the output index of the target node.
    pub fn set_input_input_12_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(12, (target.get_id(), output_index));
        self
    }

    /// Connects to input 13: ""
    pub fn set_input_input_13<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(13, (target.get_id(), 0));
        self
    }

    /// Connects to input 13: "" and specifies the output index of the target node.
    pub fn set_input_input_13_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(13, (target.get_id(), output_index));
        self
    }

    /// Connects to input 14: ""
    pub fn set_input_input_14<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(14, (target.get_id(), 0));
        self
    }

    /// Connects to input 14: "" and specifies the output index of the target node.
    pub fn set_input_input_14_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(14, (target.get_id(), output_index));
        self
    }

    /// Connects to input 15: ""
    pub fn set_input_input_15<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(15, (target.get_id(), 0));
        self
    }

    /// Connects to input 15: "" and specifies the output index of the target node.
    pub fn set_input_input_15_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(15, (target.get_id(), output_index));
        self
    }

    /// Connects to input 16: ""
    pub fn set_input_input_16<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(16, (target.get_id(), 0));
        self
    }

    /// Connects to input 16: "" and specifies the output index of the target node.
    pub fn set_input_input_16_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(16, (target.get_id(), output_index));
        self
    }

    /// Connects to input 17: ""
    pub fn set_input_input_17<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(17, (target.get_id(), 0));
        self
    }

    /// Connects to input 17: "" and specifies the output index of the target node.
    pub fn set_input_input_17_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(17, (target.get_id(), output_index));
        self
    }

    /// Connects to input 18: ""
    pub fn set_input_input_18<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(18, (target.get_id(), 0));
        self
    }

    /// Connects to input 18: "" and specifies the output index of the target node.
    pub fn set_input_input_18_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(18, (target.get_id(), output_index));
        self
    }

    /// Connects to input 19: ""
    pub fn set_input_input_19<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(19, (target.get_id(), 0));
        self
    }

    /// Connects to input 19: "" and specifies the output index of the target node.
    pub fn set_input_input_19_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(19, (target.get_id(), output_index));
        self
    }

    /// Connects to input 20: ""
    pub fn set_input_input_20<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(20, (target.get_id(), 0));
        self
    }

    /// Connects to input 20: "" and specifies the output index of the target node.
    pub fn set_input_input_20_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(20, (target.get_id(), output_index));
        self
    }

    /// Connects to input 21: ""
    pub fn set_input_input_21<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(21, (target.get_id(), 0));
        self
    }

    /// Connects to input 21: "" and specifies the output index of the target node.
    pub fn set_input_input_21_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(21, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_lightness(mut self, val: f32) -> Self {
        self.params.insert(
            "lightness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lightness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lightness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variance(mut self, val: f32) -> Self {
        self.params.insert(
            "variance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_variance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variance".to_string(),
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
    pub fn with_wavy(mut self, val: f32) -> Self {
        self.params.insert(
            "wavy".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wavy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wavy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ringy(mut self, val: f32) -> Self {
        self.params.insert(
            "ringy".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ringy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ringy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_grainy(mut self, val: f32) -> Self {
        self.params.insert(
            "grainy".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_grainy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grainy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale3d(mut self, val: f32) -> Self {
        self.params.insert(
            "scale3d".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale3d_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale3d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bumpscale(mut self, val: f32) -> Self {
        self.params.insert(
            "bumpscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bumpscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bumpscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scuffscale(mut self, val: f32) -> Self {
        self.params.insert(
            "scuffscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scuffscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scuffscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_grainscale(mut self, val: f32) -> Self {
        self.params.insert(
            "grainscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_grainscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grainscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_plankwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "plankwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_plankwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "plankwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_groutwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "groutwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_groutwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "groutwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_darkcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "darkcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_darkcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "darkcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_groovecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "groovecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_groovecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "groovecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_speccolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "speccolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_speccolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speccolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reflcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "reflcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_reflcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopWoodplank {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "woodplank"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopWorleynoiseFour {
    TwoPoints = 0,
    FourPoints = 1,
}

#[derive(Debug, Clone)]
pub struct VopWorleynoise {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopWorleynoise {
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

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float(val),
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
    pub fn with_offset(mut self, val: f32) -> Self {
        self.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float(val),
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

    // --- Float2 parameters ---
    pub fn with_pos_u(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "pos_u".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pos_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos_u".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_freq_u(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "freq_u".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_freq_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "freq_u".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset_u(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "offset_u".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_offset_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset_u".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_pos_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos_v".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos_v".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_freq_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "freq_v".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_freq_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "freq_v".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset_v".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset_v".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_pos_v4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "pos_v4".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_pos_v4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos_v4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_freq_v4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "freq_v4".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_freq_v4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "freq_v4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset_v4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "offset_v4".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_offset_v4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset_v4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_four(mut self, val: VopWorleynoiseFour) -> Self {
        self.params.insert(
            "four".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_four_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "four".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_metric(mut self, val: &str) -> Self {
        self.params.insert(
            "metric".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_metric_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "metric".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopWorleynoise {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "worleynoise"
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
pub struct VopWoven {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl VopWoven {
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

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_warp(mut self, val: f32) -> Self {
        self.params.insert(
            "warp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_warp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "warp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weft(mut self, val: f32) -> Self {
        self.params.insert(
            "weft".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weft_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weft".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_yarn(mut self, val: f32) -> Self {
        self.params.insert(
            "yarn".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_yarn_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yarn".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopWoven {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "woven"
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
