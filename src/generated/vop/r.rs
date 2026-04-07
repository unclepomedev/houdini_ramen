#[derive(Debug, Clone)]
pub struct VopRadtodeg {
    pub base: crate::core::types::NodeBase,
}

impl VopRadtodeg {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_rad(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rad_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rad_uf(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rad_uf".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rad_uf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rad_uf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_rad_u(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "rad_u".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_rad_u_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rad_u".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_rad_v(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "rad_v".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rad_v_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rad_v".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rad_p(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "rad_p".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rad_p_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rad_p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rad_n(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "rad_n".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rad_n_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rad_n".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rad_c(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "rad_c".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rad_c_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rad_c".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rad_uv(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "rad_uv".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rad_uv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rad_uv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rad_up(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "rad_up".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rad_up_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rad_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rad_un(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "rad_un".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rad_un_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rad_un".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rad_uc(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "rad_uc".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rad_uc_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rad_uc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_rad_v4(mut self, val: [f32; 4]) -> Self {
        self.base.params.insert(
            "rad_v4".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_rad_v4_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rad_v4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRadtodeg {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "radtodeg"
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
pub struct VopRainbow {
    pub base: crate::core::types::NodeBase,
}

impl VopRainbow {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_sat(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sat".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sat_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRainbow {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rainbow"
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
pub struct VopRampfilter {
    pub base: crate::core::types::NodeBase,
}

impl VopRampfilter {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_gammaval(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gammaval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gammaval_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gammaval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rangemin(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rangemin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rangemin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rangemin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rangemax(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rangemax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rangemax_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rangemax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spacename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "spacename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_spacename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spacename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gammaname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "gammaname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_gammaname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gammaname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lbt(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lbt".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lbt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lbt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rbt(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rbt".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rbt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rbt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ftype(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ftype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ftype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ftype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_active(mut self, val: bool) -> Self {
        self.base.params.insert(
            "active".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_active_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "active".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRampfilter {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rampfilter"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopRampparmRamptype {
    RgbColorRamp = 0,
    /// Spline Ramp (float)
    SplineRampFloat = 1,
}

#[derive(Debug, Clone)]
pub struct VopRampparm {
    pub base: crate::core::types::NodeBase,
}

impl VopRampparm {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Menu parameters ---
    pub fn with_ramptype(mut self, val: VopRampparmRamptype) -> Self {
        self.base.params.insert(
            "ramptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_ramptype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ramptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_rampcolordefault(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "rampcolordefault".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_rampcolordefault_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rampcolordefault".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rampfloatdefault(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "rampfloatdefault".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_rampfloatdefault_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rampfloatdefault".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_parmscope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "parmscope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parmscope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmscope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmaccess(mut self, val: &str) -> Self {
        self.base.params.insert(
            "parmaccess".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parmaccess_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmaccess".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "parmname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parmname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmprefix(mut self, val: &str) -> Self {
        self.base.params.insert(
            "parmprefix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parmprefix_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmpostfix(mut self, val: &str) -> Self {
        self.base.params.insert(
            "parmpostfix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parmpostfix_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmpostfix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmlabel(mut self, val: &str) -> Self {
        self.base.params.insert(
            "parmlabel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parmlabel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rampcolortype(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rampcolortype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rampcolortype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rampcolortype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rampbasisdefault(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rampbasisdefault".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rampbasisdefault_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rampbasisdefault".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disablewhen(mut self, val: &str) -> Self {
        self.base.params.insert(
            "disablewhen".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_disablewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "disablewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hidewhen(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hidewhen".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hidewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hidewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_callback(mut self, val: &str) -> Self {
        self.base.params.insert(
            "callback".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_callback_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "callback".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_help(mut self, val: &str) -> Self {
        self.base.params.insert(
            "help".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_help_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "help".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useasparmdefiner(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useasparmdefiner".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useasparmdefiner_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useasparmdefiner".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rampshowcontrolsdefault(mut self, val: bool) -> Self {
        self.base.params.insert(
            "rampshowcontrolsdefault".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rampshowcontrolsdefault_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rampshowcontrolsdefault".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRampparm {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rampparm"
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
pub struct VopRamps {
    pub base: crate::core::types::NodeBase,
}

impl VopRamps {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
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
    pub fn with_offset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blur(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_mirror(mut self, val: bool) -> Self {
        self.base.params.insert(
            "mirror".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mirror_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mirror".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRamps {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ramps"
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
pub struct VopRandom {
    pub base: crate::core::types::NodeBase,
}

impl VopRandom {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_pos(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos_uf1d(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pos_uf1d".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_uf1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos_uf1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_pos_v1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pos_v1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos_v1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos_v1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos_p1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pos_p1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos_p1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos_p1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos_n1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pos_n1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos_n1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos_n1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos_c1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pos_c1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos_c1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos_c1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos_uv1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pos_uv1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos_uv1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos_uv1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos_up1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pos_up1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos_up1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos_up1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos_un1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pos_un1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos_un1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos_un1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos_uc1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pos_uc1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos_uc1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos_uc1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_pos_v41d(mut self, val: [f32; 4]) -> Self {
        self.base.params.insert(
            "pos_v41d".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_pos_v41d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos_v41d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pos_i(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pos_i".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pos_i_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos_i".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_roundtoint(mut self, val: bool) -> Self {
        self.base.params.insert(
            "roundtoint".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_roundtoint_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roundtoint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRandom {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "random"
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
pub struct VopRandomBrj {
    pub base: crate::core::types::NodeBase,
}

impl VopRandomBrj {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_seed(mut self, val: f32) -> Self {
        self.base.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed_uf1d(mut self, val: f32) -> Self {
        self.base.params.insert(
            "seed_uf1d".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seed_uf1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_uf1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_seed_v1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "seed_v1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_seed_v1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_v1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed_p1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "seed_p1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_seed_p1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_p1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed_n1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "seed_n1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_seed_n1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_n1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed_c1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "seed_c1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_seed_c1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_c1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed_uv1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "seed_uv1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_seed_uv1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_uv1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed_up1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "seed_up1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_seed_up1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_up1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed_un1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "seed_un1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_seed_un1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_un1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed_uc1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "seed_uc1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_seed_uc1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_uc1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_seed_v41d(mut self, val: [f32; 4]) -> Self {
        self.base.params.insert(
            "seed_v41d".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_seed_v41d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_v41d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_seed_i(mut self, val: i32) -> Self {
        self.base.params.insert(
            "seed_i".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_seed_i_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_i".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset(mut self, val: i32) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRandomBrj {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "random_brj"
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
pub struct VopRandomSobol {
    pub base: crate::core::types::NodeBase,
}

impl VopRandomSobol {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_seed(mut self, val: f32) -> Self {
        self.base.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed_uf1d(mut self, val: f32) -> Self {
        self.base.params.insert(
            "seed_uf1d".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seed_uf1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_uf1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_seed_v1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "seed_v1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_seed_v1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_v1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed_p1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "seed_p1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_seed_p1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_p1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed_n1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "seed_n1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_seed_n1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_n1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed_c1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "seed_c1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_seed_c1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_c1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed_uv1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "seed_uv1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_seed_uv1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_uv1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed_up1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "seed_up1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_seed_up1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_up1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed_un1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "seed_un1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_seed_un1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_un1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed_uc1d(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "seed_uc1d".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_seed_uc1d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_uc1d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_seed_v41d(mut self, val: [f32; 4]) -> Self {
        self.base.params.insert(
            "seed_v41d".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_seed_v41d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_v41d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_seed_i(mut self, val: i32) -> Self {
        self.base.params.insert(
            "seed_i".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_seed_i_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed_i".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset(mut self, val: i32) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRandomSobol {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "random_sobol"
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
pub struct VopRaybounce {
    pub base: crate::core::types::NodeBase,
}

impl VopRaybounce {
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

impl crate::core::types::HoudiniNode for VopRaybounce {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "raybounce"
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
pub struct VopRayhit {
    pub base: crate::core::types::NodeBase,
}

impl VopRayhit {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_bias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_dir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "dir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRayhit {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rayhit"
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
pub struct VopRayimport {
    pub base: crate::core::types::NodeBase,
}

impl VopRayimport {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_var(mut self, val: &str) -> Self {
        self.base.params.insert(
            "var".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_var_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "var".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRayimport {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rayimport"
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
pub struct VopRaytrace {
    pub base: crate::core::types::NodeBase,
}

impl VopRaytrace {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_bias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxcontrib(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxcontrib".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxcontrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxcontrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "angle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "angle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_ndir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "ndir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ndir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ndir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_samples(mut self, val: i32) -> Self {
        self.base.params.insert(
            "samples".to_string(),
            crate::core::types::ParamValue::Int(val),
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

    // --- String parameters ---
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRaytrace {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "raytrace"
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
pub struct VopRayweight {
    pub base: crate::core::types::NodeBase,
}

impl VopRayweight {
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

impl crate::core::types::HoudiniNode for VopRayweight {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rayweight"
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
pub struct VopReflect {
    pub base: crate::core::types::NodeBase,
}

impl VopReflect {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_dir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "dir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nml(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "nml".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_nml_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "nml".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopReflect {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "reflect"
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
pub struct VopReflective {
    pub base: crate::core::types::NodeBase,
}

impl VopReflective {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_rbias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rbias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_rcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "rcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_envcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "envcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_envcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "envcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_envmap(mut self, val: &str) -> Self {
        self.base.params.insert(
            "envmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_envmap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "envmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_facefwd(mut self, val: bool) -> Self {
        self.base.params.insert(
            "facefwd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_facefwd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "facefwd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopReflective {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "reflective"
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
pub struct VopReflectlight {
    pub base: crate::core::types::NodeBase,
}

impl VopReflectlight {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_bias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxcontrib(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxcontrib".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxcontrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxcontrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "angle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "angle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_samples(mut self, val: i32) -> Self {
        self.base.params.insert(
            "samples".to_string(),
            crate::core::types::ParamValue::Int(val),
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

    // --- String parameters ---
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopReflectlight {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "reflectlight"
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
pub struct VopRefract {
    pub base: crate::core::types::NodeBase,
}

impl VopRefract {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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
    pub fn with_dir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "dir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nn(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "nn".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_nn_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "nn".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRefract {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "refract"
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
pub struct VopRefractlight {
    pub base: crate::core::types::NodeBase,
}

impl VopRefractlight {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_bias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxcontrib(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxcontrib".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxcontrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxcontrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "angle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "angle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_samples(mut self, val: i32) -> Self {
        self.base.params.insert(
            "samples".to_string(),
            crate::core::types::ParamValue::Int(val),
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

    // --- String parameters ---
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRefractlight {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "refractlight"
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
pub struct VopRegexFind {
    pub base: crate::core::types::NodeBase,
}

impl VopRegexFind {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_start(mut self, val: i32) -> Self {
        self.base.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_end(mut self, val: i32) -> Self {
        self.base
            .params
            .insert("end".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_regex_findtype(mut self, val: &str) -> Self {
        self.base.params.insert(
            "regex_findtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_regex_findtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "regex_findtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_regex(mut self, val: &str) -> Self {
        self.base.params.insert(
            "regex".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_regex_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "regex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_string(mut self, val: &str) -> Self {
        self.base.params.insert(
            "string".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_string_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "string".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRegexFind {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "regex_find"
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
pub struct VopRegexFindall {
    pub base: crate::core::types::NodeBase,
}

impl VopRegexFindall {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_start(mut self, val: i32) -> Self {
        self.base.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_end(mut self, val: i32) -> Self {
        self.base
            .params
            .insert("end".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_regex_findalltype(mut self, val: &str) -> Self {
        self.base.params.insert(
            "regex_findalltype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_regex_findalltype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "regex_findalltype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_regex(mut self, val: &str) -> Self {
        self.base.params.insert(
            "regex".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_regex_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "regex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_string(mut self, val: &str) -> Self {
        self.base.params.insert(
            "string".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_string_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "string".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRegexFindall {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "regex_findall"
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
pub struct VopRegexMatch {
    pub base: crate::core::types::NodeBase,
}

impl VopRegexMatch {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_regex(mut self, val: &str) -> Self {
        self.base.params.insert(
            "regex".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_regex_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "regex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_string(mut self, val: &str) -> Self {
        self.base.params.insert(
            "string".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_string_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "string".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRegexMatch {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "regex_match"
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
pub struct VopRegexReplace {
    pub base: crate::core::types::NodeBase,
}

impl VopRegexReplace {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_maxreplace(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxreplace".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxreplace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxreplace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_find_regex(mut self, val: &str) -> Self {
        self.base.params.insert(
            "find_regex".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_find_regex_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "find_regex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_replace_regex(mut self, val: &str) -> Self {
        self.base.params.insert(
            "replace_regex".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_replace_regex_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "replace_regex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_string(mut self, val: &str) -> Self {
        self.base.params.insert(
            "string".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_string_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "string".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRegexReplace {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "regex_replace"
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
pub struct VopRegexSplit {
    pub base: crate::core::types::NodeBase,
}

impl VopRegexSplit {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_maxplit(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxplit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxplit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxplit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_regex(mut self, val: &str) -> Self {
        self.base.params.insert(
            "regex".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_regex_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "regex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_string(mut self, val: &str) -> Self {
        self.base.params.insert(
            "string".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_string_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "string".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRegexSplit {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "regex_split"
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
pub struct VopRelbbox {
    pub base: crate::core::types::NodeBase,
}

impl VopRelbbox {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_opinput(mut self, val: &str) -> Self {
        self.base.params.insert(
            "opinput".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opinput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "opinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.base.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRelbbox {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "relbbox"
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
pub struct VopRelpointbbox {
    pub base: crate::core::types::NodeBase,
}

impl VopRelpointbbox {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_opinput(mut self, val: &str) -> Self {
        self.base.params.insert(
            "opinput".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opinput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "opinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.base.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRelpointbbox {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "relpointbbox"
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
pub struct VopRemoveindex {
    pub base: crate::core::types::NodeBase,
}

impl VopRemoveindex {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_defaultvalue(mut self, val: f32) -> Self {
        self.base.params.insert(
            "defaultvalue".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_defaultvalue_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_defaultvalue_s5(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "defaultvalue_s5".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_defaultvalue_s5_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultvalue_s5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_defaultvalue_s4(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "defaultvalue_s4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_defaultvalue_s4_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultvalue_s4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_defaultvalue_s3(mut self, val: [f32; 4]) -> Self {
        self.base.params.insert(
            "defaultvalue_s3".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_defaultvalue_s3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultvalue_s3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultvalue_s6(mut self, val: [f32; 4]) -> Self {
        self.base.params.insert(
            "defaultvalue_s6".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_defaultvalue_s6_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultvalue_s6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- FloatArray parameters ---
    pub fn with_defaultvalue_s7(mut self, val: Vec<f32>) -> Self {
        self.base.params.insert(
            "defaultvalue_s7".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_defaultvalue_s7_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultvalue_s7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultvalue_s8(mut self, val: Vec<f32>) -> Self {
        self.base.params.insert(
            "defaultvalue_s8".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_defaultvalue_s8_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultvalue_s8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_defaultvalue_s1(mut self, val: i32) -> Self {
        self.base.params.insert(
            "defaultvalue_s1".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_defaultvalue_s1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultvalue_s1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultvalue_s2(mut self, val: &str) -> Self {
        self.base.params.insert(
            "defaultvalue_s2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_defaultvalue_s2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultvalue_s2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRemoveindex {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "removeindex"
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
pub struct VopRemovekey {
    pub base: crate::core::types::NodeBase,
}

impl VopRemovekey {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }
}

impl crate::core::types::HoudiniNode for VopRemovekey {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "removekey"
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
pub struct VopRemovepoint {
    pub base: crate::core::types::NodeBase,
}

impl VopRemovepoint {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_handle(mut self, val: i32) -> Self {
        self.base.params.insert(
            "handle".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_handle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "handle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ptnum(mut self, val: i32) -> Self {
        self.base.params.insert(
            "ptnum".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ptnum_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ptnum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_removeprims(mut self, val: bool) -> Self {
        self.base.params.insert(
            "removeprims".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removeprims_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "removeprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRemovepoint {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "removepoint"
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
pub struct VopRemoveprim {
    pub base: crate::core::types::NodeBase,
}

impl VopRemoveprim {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_handle(mut self, val: i32) -> Self {
        self.base.params.insert(
            "handle".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_handle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "handle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primnum(mut self, val: i32) -> Self {
        self.base.params.insert(
            "primnum".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primnum_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primnum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_andpoints(mut self, val: i32) -> Self {
        self.base.params.insert(
            "andpoints".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_andpoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "andpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRemoveprim {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "removeprim"
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
pub struct VopRemovevalue {
    pub base: crate::core::types::NodeBase,
}

impl VopRemovevalue {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_valuetoremove(mut self, val: f32) -> Self {
        self.base.params.insert(
            "valuetoremove".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_valuetoremove_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "valuetoremove".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_valuetoremove_s5(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "valuetoremove_s5".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_valuetoremove_s5_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "valuetoremove_s5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_valuetoremove_s4(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "valuetoremove_s4".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_valuetoremove_s4_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "valuetoremove_s4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_valuetoremove_s3(mut self, val: [f32; 4]) -> Self {
        self.base.params.insert(
            "valuetoremove_s3".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_valuetoremove_s3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "valuetoremove_s3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_valuetoremove_s6(mut self, val: [f32; 4]) -> Self {
        self.base.params.insert(
            "valuetoremove_s6".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_valuetoremove_s6_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "valuetoremove_s6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- FloatArray parameters ---
    pub fn with_valuetoremove_s7(mut self, val: Vec<f32>) -> Self {
        self.base.params.insert(
            "valuetoremove_s7".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_valuetoremove_s7_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "valuetoremove_s7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_valuetoremove_s8(mut self, val: Vec<f32>) -> Self {
        self.base.params.insert(
            "valuetoremove_s8".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_valuetoremove_s8_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "valuetoremove_s8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_valuetoremove_s1(mut self, val: i32) -> Self {
        self.base.params.insert(
            "valuetoremove_s1".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_valuetoremove_s1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "valuetoremove_s1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_valuetoremove_s2(mut self, val: &str) -> Self {
        self.base.params.insert(
            "valuetoremove_s2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_valuetoremove_s2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "valuetoremove_s2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRemovevalue {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "removevalue"
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
pub struct VopRenderstate {
    pub base: crate::core::types::NodeBase,
}

impl VopRenderstate {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_default_f(mut self, val: f32) -> Self {
        self.base.params.insert(
            "default_f".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_default_f_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_default_v(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "default_v".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_default_v_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_v".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_default_i(mut self, val: i32) -> Self {
        self.base.params.insert(
            "default_i".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_default_i_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_i".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_var(mut self, val: &str) -> Self {
        self.base.params.insert(
            "var".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_var_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "var".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_default(mut self, val: &str) -> Self {
        self.base.params.insert(
            "default".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_default_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRenderstate {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "renderstate"
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
pub struct VopReorder {
    pub base: crate::core::types::NodeBase,
}

impl VopReorder {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopReorder {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "reorder"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopReporterrorReportas {
    Nothing = 0,
    Warning = 1,
    Error = 2,
}

#[derive(Debug, Clone)]
pub struct VopReporterror {
    pub base: crate::core::types::NodeBase,
}

impl VopReporterror {
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

    // --- Menu parameters ---
    pub fn with_reportas(mut self, val: VopReporterrorReportas) -> Self {
        self.base.params.insert(
            "reportas".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_reportas_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reportas".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_format(mut self, val: &str) -> Self {
        self.base.params.insert(
            "format".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_format_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "format".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopReporterror {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "reporterror"
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
pub struct VopRes {
    pub base: crate::core::types::NodeBase,
}

impl VopRes {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_input_index(mut self, val: i32) -> Self {
        self.base.params.insert(
            "input_index".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_input_index_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "input_index".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRes {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "res"
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
pub struct VopReshapevalue {
    pub base: crate::core::types::NodeBase,
}

impl VopReshapevalue {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: ""
    pub fn set_input_input_11(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "" and specifies the output index of the target node.
    pub fn set_input_input_11_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    /// Connects to input 12: ""
    pub fn set_input_input_12(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(12, (target.get_id(), 0));
        self
    }

    /// Connects to input 12: "" and specifies the output index of the target node.
    pub fn set_input_input_12_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(12, (target.get_id(), output_index));
        self
    }

    /// Connects to input 13: ""
    pub fn set_input_input_13(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(13, (target.get_id(), 0));
        self
    }

    /// Connects to input 13: "" and specifies the output index of the target node.
    pub fn set_input_input_13_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(13, (target.get_id(), output_index));
        self
    }

    /// Connects to input 14: ""
    pub fn set_input_input_14(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(14, (target.get_id(), 0));
        self
    }

    /// Connects to input 14: "" and specifies the output index of the target node.
    pub fn set_input_input_14_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(14, (target.get_id(), output_index));
        self
    }

    /// Connects to input 15: ""
    pub fn set_input_input_15(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(15, (target.get_id(), 0));
        self
    }

    /// Connects to input 15: "" and specifies the output index of the target node.
    pub fn set_input_input_15_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(15, (target.get_id(), output_index));
        self
    }

    /// Connects to input 16: ""
    pub fn set_input_input_16(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(16, (target.get_id(), 0));
        self
    }

    /// Connects to input 16: "" and specifies the output index of the target node.
    pub fn set_input_input_16_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(16, (target.get_id(), output_index));
        self
    }

    /// Connects to input 17: ""
    pub fn set_input_input_17(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(17, (target.get_id(), 0));
        self
    }

    /// Connects to input 17: "" and specifies the output index of the target node.
    pub fn set_input_input_17_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(17, (target.get_id(), output_index));
        self
    }

    /// Connects to input 18: ""
    pub fn set_input_input_18(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(18, (target.get_id(), 0));
        self
    }

    /// Connects to input 18: "" and specifies the output index of the target node.
    pub fn set_input_input_18_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(18, (target.get_id(), output_index));
        self
    }

    /// Connects to input 19: ""
    pub fn set_input_input_19(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(19, (target.get_id(), 0));
        self
    }

    /// Connects to input 19: "" and specifies the output index of the target node.
    pub fn set_input_input_19_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(19, (target.get_id(), output_index));
        self
    }

    /// Connects to input 20: ""
    pub fn set_input_input_20(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(20, (target.get_id(), 0));
        self
    }

    /// Connects to input 20: "" and specifies the output index of the target node.
    pub fn set_input_input_20_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(20, (target.get_id(), output_index));
        self
    }

    /// Connects to input 21: ""
    pub fn set_input_input_21(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(21, (target.get_id(), 0));
        self
    }

    /// Connects to input 21: "" and specifies the output index of the target node.
    pub fn set_input_input_21_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(21, (target.get_id(), output_index));
        self
    }

    /// Connects to input 22: ""
    pub fn set_input_input_22(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(22, (target.get_id(), 0));
        self
    }

    /// Connects to input 22: "" and specifies the output index of the target node.
    pub fn set_input_input_22_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(22, (target.get_id(), output_index));
        self
    }

    /// Connects to input 23: ""
    pub fn set_input_input_23(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(23, (target.get_id(), 0));
        self
    }

    /// Connects to input 23: "" and specifies the output index of the target node.
    pub fn set_input_input_23_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(23, (target.get_id(), output_index));
        self
    }

    /// Connects to input 24: ""
    pub fn set_input_input_24(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(24, (target.get_id(), 0));
        self
    }

    /// Connects to input 24: "" and specifies the output index of the target node.
    pub fn set_input_input_24_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(24, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_fitsrcmin(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fitsrcmin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fitsrcmin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fitsrcmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fitsrcmax(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fitsrcmax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fitsrcmax_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fitsrcmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fitdestmin(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fitdestmin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fitdestmin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fitdestmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fitdestmax(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fitdestmax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fitdestmax_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fitdestmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_contourwidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "contourwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_contourwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "contourwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_contoursharp(mut self, val: f32) -> Self {
        self.base.params.insert(
            "contoursharp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_contoursharp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "contoursharp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sclipstart(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sclipstart".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sclipstart_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sclipstart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sclipsuppress(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sclipsuppress".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sclipsuppress_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sclipsuppress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lowlimit(mut self, val: f32) -> Self {
        self.base.params.insert(
            "lowlimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lowlimit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lowlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_highlimit(mut self, val: f32) -> Self {
        self.base.params.insert(
            "highlimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_highlimit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "highlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amp(mut self, val: f32) -> Self {
        self.base.params.insert(
            "amp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "ramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_ramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ramp_color(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "ramp_color".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_ramp_color_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ramp_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rampspace(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rampspace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rampspace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rampspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rampextl(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rampextl".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rampextl_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rampextl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rampextr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rampextr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rampextr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rampextr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ftype(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ftype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ftype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ftype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_fitenable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fitenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fitenable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fitenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rampenable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "rampenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rampenable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rampenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_contourenable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "contourenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_contourenable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "contourenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sclipenable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "sclipenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sclipenable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sclipenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lowlimitenable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "lowlimitenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_lowlimitenable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lowlimitenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_highlimitenable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "highlimitenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_highlimitenable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "highlimitenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopReshapevalue {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "reshapevalue"
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
pub struct VopRestpos {
    pub base: crate::core::types::NodeBase,
}

impl VopRestpos {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_space(mut self, val: &str) -> Self {
        self.base.params.insert(
            "space".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_space_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "space".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRestpos {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "restpos"
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
pub struct VopReverse {
    pub base: crate::core::types::NodeBase,
}

impl VopReverse {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopReverse {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "reverse"
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
pub struct VopRgbtohsv {
    pub base: crate::core::types::NodeBase,
}

impl VopRgbtohsv {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_rgb(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "rgb".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rgb_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rgb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rgb_uc(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "rgb_uc".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rgb_uc_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rgb_uc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRgbtohsv {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rgbtohsv"
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
pub struct VopRhexes {
    pub base: crate::core::types::NodeBase,
}

impl VopRhexes {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: ""
    pub fn set_input_input_11(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "" and specifies the output index of the target node.
    pub fn set_input_input_11_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    /// Connects to input 12: ""
    pub fn set_input_input_12(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(12, (target.get_id(), 0));
        self
    }

    /// Connects to input 12: "" and specifies the output index of the target node.
    pub fn set_input_input_12_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(12, (target.get_id(), output_index));
        self
    }

    /// Connects to input 13: ""
    pub fn set_input_input_13(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(13, (target.get_id(), 0));
        self
    }

    /// Connects to input 13: "" and specifies the output index of the target node.
    pub fn set_input_input_13_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(13, (target.get_id(), output_index));
        self
    }

    /// Connects to input 14: ""
    pub fn set_input_input_14(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(14, (target.get_id(), 0));
        self
    }

    /// Connects to input 14: "" and specifies the output index of the target node.
    pub fn set_input_input_14_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(14, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_dotsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "dotsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dotsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dotsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dotrand(mut self, val: f32) -> Self {
        self.base.params.insert(
            "dotrand".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dotrand_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dotrand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hexness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "hexness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hexness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hexness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_softness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "softness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_softness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "softness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sfreq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sfreq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tfreq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tfreq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "soff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_soff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "toff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_toff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "toff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srand(mut self, val: f32) -> Self {
        self.base.params.insert(
            "srand".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_srand_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trand(mut self, val: f32) -> Self {
        self.base.params.insert(
            "trand".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trand_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blur(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_layers(mut self, val: i32) -> Self {
        self.base.params.insert(
            "layers".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_layers_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "layers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_filter(mut self, val: &str) -> Self {
        self.base.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRhexes {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rhexes"
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
pub struct VopRings {
    pub base: crate::core::types::NodeBase,
}

impl VopRings {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: ""
    pub fn set_input_input_11(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "" and specifies the output index of the target node.
    pub fn set_input_input_11_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    /// Connects to input 12: ""
    pub fn set_input_input_12(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(12, (target.get_id(), 0));
        self
    }

    /// Connects to input 12: "" and specifies the output index of the target node.
    pub fn set_input_input_12_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(12, (target.get_id(), output_index));
        self
    }

    /// Connects to input 13: ""
    pub fn set_input_input_13(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(13, (target.get_id(), 0));
        self
    }

    /// Connects to input 13: "" and specifies the output index of the target node.
    pub fn set_input_input_13_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(13, (target.get_id(), output_index));
        self
    }

    /// Connects to input 14: ""
    pub fn set_input_input_14(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(14, (target.get_id(), 0));
        self
    }

    /// Connects to input 14: "" and specifies the output index of the target node.
    pub fn set_input_input_14_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(14, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_dotsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "dotsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dotsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dotsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dotrand(mut self, val: f32) -> Self {
        self.base.params.insert(
            "dotrand".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dotrand_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dotrand".to_string(),
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
    pub fn with_widthrand(mut self, val: f32) -> Self {
        self.base.params.insert(
            "widthrand".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_widthrand_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "widthrand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sfreq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sfreq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tfreq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tfreq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "soff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_soff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "toff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_toff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "toff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srand(mut self, val: f32) -> Self {
        self.base.params.insert(
            "srand".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_srand_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trand(mut self, val: f32) -> Self {
        self.base.params.insert(
            "trand".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trand_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blur(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_layers(mut self, val: i32) -> Self {
        self.base.params.insert(
            "layers".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_layers_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "layers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_filter(mut self, val: &str) -> Self {
        self.base.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRings {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rings"
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
pub struct VopRint {
    pub base: crate::core::types::NodeBase,
}

impl VopRint {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_val(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val_uf(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val_uf".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val_uf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val_uf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_val_u(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "val_u".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_val_u_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val_u".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_val_v(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "val_v".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_v_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val_v".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val_p(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "val_p".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_p_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val_p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val_n(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "val_n".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_n_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val_n".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val_c(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "val_c".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_c_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val_c".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val_uv(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "val_uv".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_uv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val_uv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val_up(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "val_up".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_up_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val_un(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "val_un".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_un_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val_un".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val_uc(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "val_uc".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_uc_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val_uc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_val_v4(mut self, val: [f32; 4]) -> Self {
        self.base.params.insert(
            "val_v4".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_val_v4_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val_v4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRint {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rint"
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
pub struct VopRipples {
    pub base: crate::core::types::NodeBase,
}

impl VopRipples {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_sfreq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sfreq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tfreq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tfreq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "soff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_soff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "toff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_toff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "toff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_decay(mut self, val: f32) -> Self {
        self.base.params.insert(
            "decay".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_decay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "decay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_waveoff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "waveoff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_waveoff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "waveoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRipples {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ripples"
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
pub struct VopRisRslmaterial {
    pub base: crate::core::types::NodeBase,
}

impl VopRisRslmaterial {
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

impl crate::core::types::HoudiniNode for VopRisRslmaterial {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ris_RSLmaterial"
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
pub struct VopRotate {
    pub base: crate::core::types::NodeBase,
}

impl VopRotate {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_angle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "angle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "angle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_axis(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "axis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_axis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "axis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- FloatArray parameters ---
    pub fn with_mat(mut self, val: Vec<f32>) -> Self {
        self.base.params.insert(
            "mat".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_mat_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mat_m3(mut self, val: Vec<f32>) -> Self {
        self.base.params.insert(
            "mat_m3".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_mat_m3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mat_m3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mat_um(mut self, val: Vec<f32>) -> Self {
        self.base.params.insert(
            "mat_um".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_mat_um_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mat_um".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRotate {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rotate"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopRoundededgeMode {
    ConcaveAndConvexEdges = 0,
    ConcaveEdges = 1,
    ConvexEdges = 2,
}

#[derive(Debug, Clone)]
pub struct VopRoundededge {
    pub base: crate::core::types::NodeBase,
}

impl VopRoundededge {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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
    pub fn with_cuspangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "cuspangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cuspangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cuspangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_samples(mut self, val: i32) -> Self {
        self.base.params.insert(
            "samples".to_string(),
            crate::core::types::ParamValue::Int(val),
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

    // --- Menu parameters ---
    pub fn with_mode(mut self, val: VopRoundededgeMode) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRoundededge {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "roundededge"
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
pub struct VopRslBias {
    pub base: crate::core::types::NodeBase,
}

impl VopRslBias {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_val(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount(mut self, val: f32) -> Self {
        self.base.params.insert(
            "amount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslBias {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_bias"
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
pub struct VopRslCalculatenormal {
    pub base: crate::core::types::NodeBase,
}

impl VopRslCalculatenormal {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_pos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos_uv(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pos_uv".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos_uv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos_uv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslCalculatenormal {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_calculatenormal"
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
pub struct VopRslCtransform {
    pub base: crate::core::types::NodeBase,
}

impl VopRslCtransform {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_iclr(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "iclr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iclr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "iclr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_iclr_uc(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "iclr_uc".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_iclr_uc_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "iclr_uc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fromspace(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fromspace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fromspace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fromspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tospace(mut self, val: &str) -> Self {
        self.base.params.insert(
            "tospace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tospace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tospace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslCtransform {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_ctransform"
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
pub struct VopRslDepth {
    pub base: crate::core::types::NodeBase,
}

impl VopRslDepth {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_p1(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "p1".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "p1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslDepth {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_depth"
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
pub struct VopRslDeriv {
    pub base: crate::core::types::NodeBase,
}

impl VopRslDeriv {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslDeriv {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_deriv"
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
pub struct VopRslDudv {
    pub base: crate::core::types::NodeBase,
}

impl VopRslDudv {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_func(mut self, val: &str) -> Self {
        self.base.params.insert(
            "func".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_func_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "func".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslDudv {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_dudv"
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
pub struct VopRslEnvironment {
    pub base: crate::core::types::NodeBase,
}

impl VopRslEnvironment {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: ""
    pub fn set_input_input_11(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "" and specifies the output index of the target node.
    pub fn set_input_input_11_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_sblur(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sblur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sblur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tblur(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tblur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tblur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "swidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_swidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "swidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_twidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "twidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_twidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "twidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fill".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fill_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fill".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("R".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "R".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r1(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "R1".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "R1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r2(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "R2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "R2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r3(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "R3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "R3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map(mut self, val: &str) -> Self {
        self.base.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter(mut self, val: &str) -> Self {
        self.base.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_lerp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "lerp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_lerp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lerp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslEnvironment {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_environment"
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
pub struct VopRslGain {
    pub base: crate::core::types::NodeBase,
}

impl VopRslGain {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_val(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amount(mut self, val: f32) -> Self {
        self.base.params.insert(
            "amount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslGain {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_gain"
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
pub struct VopRslIlluminance {
    pub base: crate::core::types::NodeBase,
}

impl VopRslIlluminance {
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

    // --- Toggle parameters ---
    pub fn with_usecategories(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usecategories".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecategories_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usecategories".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslIlluminance {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_illuminance"
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
pub struct VopRslIlluminate {
    pub base: crate::core::types::NodeBase,
}

impl VopRslIlluminate {
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

    // --- Toggle parameters ---
    pub fn with_do_solar(mut self, val: bool) -> Self {
        self.base.params.insert(
            "do_solar".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_do_solar_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "do_solar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_do_cone(mut self, val: bool) -> Self {
        self.base.params.insert(
            "do_cone".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_do_cone_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "do_cone".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslIlluminate {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_illuminate"
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
pub struct VopRslImport {
    pub base: crate::core::types::NodeBase,
}

impl VopRslImport {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_default(mut self, val: f32) -> Self {
        self.base.params.insert(
            "default".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_default_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_default_uf(mut self, val: f32) -> Self {
        self.base.params.insert(
            "default_uf".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_default_uf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_uf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_default_c(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "default_c".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_default_c_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_c".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_default_uc(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "default_uc".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_default_uc_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_uc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_default_p(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "default_p".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_default_p_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_default_up(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "default_up".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_default_up_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_default_n(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "default_n".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_default_n_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_n".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_default_un(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "default_un".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_default_un_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_un".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_default_v(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "default_v".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_default_v_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_v".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_default_uv(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "default_uv".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_default_uv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_uv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- FloatArray parameters ---
    pub fn with_default_m(mut self, val: Vec<f32>) -> Self {
        self.base.params.insert(
            "default_m".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_default_m_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_m".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_default_um(mut self, val: Vec<f32>) -> Self {
        self.base.params.insert(
            "default_um".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_default_um_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_um".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_func(mut self, val: &str) -> Self {
        self.base.params.insert(
            "func".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_func_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "func".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_var(mut self, val: &str) -> Self {
        self.base.params.insert(
            "var".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_var_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "var".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_default_s(mut self, val: &str) -> Self {
        self.base.params.insert(
            "default_s".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_default_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslImport {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_import"
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
pub struct VopRslIndirectdiffuse {
    pub base: crate::core::types::NodeBase,
}

impl VopRslIndirectdiffuse {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: ""
    pub fn set_input_input_11(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "" and specifies the output index of the target node.
    pub fn set_input_input_11_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    /// Connects to input 12: ""
    pub fn set_input_input_12(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(12, (target.get_id(), 0));
        self
    }

    /// Connects to input 12: "" and specifies the output index of the target node.
    pub fn set_input_input_12_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(12, (target.get_id(), output_index));
        self
    }

    /// Connects to input 13: ""
    pub fn set_input_input_13(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(13, (target.get_id(), 0));
        self
    }

    /// Connects to input 13: "" and specifies the output index of the target node.
    pub fn set_input_input_13_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(13, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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
    pub fn with_maxdist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxdist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxdist".to_string(),
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
    pub fn with_bias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxerror(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxerror".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxerror_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxerror".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxpixeldist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxpixeldist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxpixeldist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxpixeldist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("P".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "P".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_n(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("N".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_n_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "N".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_coordsystem(mut self, val: &str) -> Self {
        self.base.params.insert(
            "coordsystem".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coordsystem_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coordsystem".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_environmentmap(mut self, val: &str) -> Self {
        self.base.params.insert(
            "environmentmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_environmentmap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "environmentmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_environmentspace(mut self, val: &str) -> Self {
        self.base.params.insert(
            "environmentspace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_environmentspace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "environmentspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_adaptive(mut self, val: bool) -> Self {
        self.base.params.insert(
            "adaptive".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adaptive_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adaptive".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloff(mut self, val: bool) -> Self {
        self.base.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_falloff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useopacity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useopacity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useopacity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useopacity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslIndirectdiffuse {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_indirectdiffuse"
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
pub struct VopRslLog {
    pub base: crate::core::types::NodeBase,
}

impl VopRslLog {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_val(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val_uf(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val_uf".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val_uf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val_uf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_base(mut self, val: f32) -> Self {
        self.base.params.insert(
            "base".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_base_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_base_uf(mut self, val: f32) -> Self {
        self.base.params.insert(
            "base_uf".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_base_uf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "base_uf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslLog {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_log"
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
pub struct VopRslOcclusion {
    pub base: crate::core::types::NodeBase,
}

impl VopRslOcclusion {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: ""
    pub fn set_input_input_11(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "" and specifies the output index of the target node.
    pub fn set_input_input_11_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    /// Connects to input 12: ""
    pub fn set_input_input_12(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(12, (target.get_id(), 0));
        self
    }

    /// Connects to input 12: "" and specifies the output index of the target node.
    pub fn set_input_input_12_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(12, (target.get_id(), output_index));
        self
    }

    /// Connects to input 13: ""
    pub fn set_input_input_13(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(13, (target.get_id(), 0));
        self
    }

    /// Connects to input 13: "" and specifies the output index of the target node.
    pub fn set_input_input_13_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(13, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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
    pub fn with_maxdist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxdist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxdist".to_string(),
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
    pub fn with_bias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxerror(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxerror".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxerror_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxerror".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxpixeldist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxpixeldist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxpixeldist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxpixeldist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("P".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "P".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_n(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("N".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_n_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "N".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_coordsystem(mut self, val: &str) -> Self {
        self.base.params.insert(
            "coordsystem".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_coordsystem_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "coordsystem".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_environmentmap(mut self, val: &str) -> Self {
        self.base.params.insert(
            "environmentmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_environmentmap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "environmentmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_environmentspace(mut self, val: &str) -> Self {
        self.base.params.insert(
            "environmentspace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_environmentspace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "environmentspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_adaptive(mut self, val: bool) -> Self {
        self.base.params.insert(
            "adaptive".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adaptive_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adaptive".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloff(mut self, val: bool) -> Self {
        self.base.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_falloff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useopacity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useopacity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useopacity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useopacity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslOcclusion {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_occlusion"
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
pub struct VopRslOglass {
    pub base: crate::core::types::NodeBase,
}

impl VopRslOglass {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: ""
    pub fn set_input_input_11(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "" and specifies the output index of the target node.
    pub fn set_input_input_11_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    /// Connects to input 12: ""
    pub fn set_input_input_12(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(12, (target.get_id(), 0));
        self
    }

    /// Connects to input 12: "" and specifies the output index of the target node.
    pub fn set_input_input_12_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(12, (target.get_id(), output_index));
        self
    }

    /// Connects to input 13: ""
    pub fn set_input_input_13(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(13, (target.get_id(), 0));
        self
    }

    /// Connects to input 13: "" and specifies the output index of the target node.
    pub fn set_input_input_13_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(13, (target.get_id(), output_index));
        self
    }

    /// Connects to input 14: ""
    pub fn set_input_input_14(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(14, (target.get_id(), 0));
        self
    }

    /// Connects to input 14: "" and specifies the output index of the target node.
    pub fn set_input_input_14_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(14, (target.get_id(), output_index));
        self
    }

    /// Connects to input 15: ""
    pub fn set_input_input_15(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(15, (target.get_id(), 0));
        self
    }

    /// Connects to input 15: "" and specifies the output index of the target node.
    pub fn set_input_input_15_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(15, (target.get_id(), output_index));
        self
    }

    /// Connects to input 16: ""
    pub fn set_input_input_16(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(16, (target.get_id(), 0));
        self
    }

    /// Connects to input 16: "" and specifies the output index of the target node.
    pub fn set_input_input_16_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(16, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_bias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitter(mut self, val: f32) -> Self {
        self.base.params.insert(
            "jitter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jitter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "jitter".to_string(),
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
    pub fn with_kt(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("Kt".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_kt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Kt".to_string(),
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
    pub fn with_angle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "angle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "angle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density(mut self, val: f32) -> Self {
        self.base.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "density".to_string(),
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
    pub fn with_xmit(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "xmit".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xmit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xmit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_atten(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "atten".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_atten_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "atten".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shadowclr(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "shadowclr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shadowclr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shadowclr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_samples(mut self, val: i32) -> Self {
        self.base.params.insert(
            "samples".to_string(),
            crate::core::types::ParamValue::Int(val),
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

    // --- String parameters ---
    pub fn with_envmap(mut self, val: &str) -> Self {
        self.base.params.insert(
            "envmap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_envmap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "envmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslOglass {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_oglass"
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
pub struct VopRslRandom {
    pub base: crate::core::types::NodeBase,
}

impl VopRslRandom {
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
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslRandom {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_random"
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
pub struct VopRslRayinfo {
    pub base: crate::core::types::NodeBase,
}

impl VopRslRayinfo {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_default(mut self, val: f32) -> Self {
        self.base.params.insert(
            "default".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_default_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_default_up(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "default_up".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_default_up_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_default_p(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "default_p".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_default_p_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_var(mut self, val: &str) -> Self {
        self.base.params.insert(
            "var".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_var_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "var".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_default_s(mut self, val: &str) -> Self {
        self.base.params.insert(
            "default_s".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_default_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslRayinfo {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_rayinfo"
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
pub struct VopRslRenderstate {
    pub base: crate::core::types::NodeBase,
}

impl VopRslRenderstate {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_default(mut self, val: f32) -> Self {
        self.base.params.insert(
            "default".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_default_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_func(mut self, val: &str) -> Self {
        self.base.params.insert(
            "func".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_func_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "func".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_var(mut self, val: &str) -> Self {
        self.base.params.insert(
            "var".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_var_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "var".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_default_s(mut self, val: &str) -> Self {
        self.base.params.insert(
            "default_s".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_default_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslRenderstate {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_renderstate"
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
pub struct VopRslShadow {
    pub base: crate::core::types::NodeBase,
}

impl VopRslShadow {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: ""
    pub fn set_input_input_11(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "" and specifies the output index of the target node.
    pub fn set_input_input_11_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_sblur(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sblur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sblur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tblur(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tblur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tblur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "swidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_swidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "swidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_twidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "twidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_twidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "twidth".to_string(),
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
    pub fn with_bias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("P".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "P".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p1(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "P1".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "P1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p2(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "P2".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "P2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p3(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "P3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "P3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map(mut self, val: &str) -> Self {
        self.base.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter(mut self, val: &str) -> Self {
        self.base.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslShadow {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_shadow"
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
pub struct VopRslStep {
    pub base: crate::core::types::NodeBase,
}

impl VopRslStep {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_min(mut self, val: f32) -> Self {
        self.base.params.insert(
            "min".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_min_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "min".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_min_uf(mut self, val: f32) -> Self {
        self.base.params.insert(
            "min_uf".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_min_uf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "min_uf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value_uf(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value_uf".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value_uf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value_uf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_min_c(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "min_c".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_min_c_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "min_c".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value_c(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "value_c".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value_c_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value_c".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_min_uc(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "min_uc".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_min_uc_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "min_uc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value_uc(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "value_uc".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_value_uc_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value_uc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslStep {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_step"
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
pub struct VopRslSurfacecolor {
    pub base: crate::core::types::NodeBase,
}

impl VopRslSurfacecolor {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_basecolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "baseColor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_basecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "baseColor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_basecolormap(mut self, val: &str) -> Self {
        self.base.params.insert(
            "baseColorMap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_basecolormap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "baseColorMap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usebasecolor(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useBaseColor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebasecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useBaseColor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepointcolor(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usePointColor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepointcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usePointColor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecolormap(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useColorMap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecolormap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useColorMap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslSurfacecolor {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_surfacecolor"
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
pub struct VopRslTexture {
    pub base: crate::core::types::NodeBase,
}

impl VopRslTexture {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: ""
    pub fn set_input_input_11(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "" and specifies the output index of the target node.
    pub fn set_input_input_11_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    /// Connects to input 12: ""
    pub fn set_input_input_12(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(12, (target.get_id(), 0));
        self
    }

    /// Connects to input 12: "" and specifies the output index of the target node.
    pub fn set_input_input_12_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(12, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_s(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("s".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_t(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("t".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ds(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("ds".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_ds_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dt(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("dt".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_dt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sblur(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sblur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sblur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tblur(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tblur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tblur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_swidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "swidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_swidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "swidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_twidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "twidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_twidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "twidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defclr_f(mut self, val: f32) -> Self {
        self.base.params.insert(
            "defclr_f".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_defclr_f_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defclr_f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fill(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fill".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fill_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fill".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_defclr(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "defclr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_defclr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defclr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_map(mut self, val: &str) -> Self {
        self.base.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter(mut self, val: &str) -> Self {
        self.base.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_lerp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "lerp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_lerp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lerp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslTexture {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_texture"
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
pub struct VopRslTextureinfo {
    pub base: crate::core::types::NodeBase,
}

impl VopRslTextureinfo {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_map(mut self, val: &str) -> Self {
        self.base.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_map_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "map".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslTextureinfo {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_textureinfo"
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
pub struct VopRslTransform {
    pub base: crate::core::types::NodeBase,
}

impl VopRslTransform {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_val(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_val_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "signature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_func(mut self, val: &str) -> Self {
        self.base.params.insert(
            "func".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_func_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "func".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fromspace(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fromspace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fromspace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fromspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tospace(mut self, val: &str) -> Self {
        self.base.params.insert(
            "tospace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tospace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tospace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRslTransform {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rsl_transform"
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
pub struct VopRstars {
    pub base: crate::core::types::NodeBase,
}

impl VopRstars {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: ""
    pub fn set_input_input_10(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "" and specifies the output index of the target node.
    pub fn set_input_input_10_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: ""
    pub fn set_input_input_11(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "" and specifies the output index of the target node.
    pub fn set_input_input_11_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    /// Connects to input 12: ""
    pub fn set_input_input_12(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(12, (target.get_id(), 0));
        self
    }

    /// Connects to input 12: "" and specifies the output index of the target node.
    pub fn set_input_input_12_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(12, (target.get_id(), output_index));
        self
    }

    /// Connects to input 13: ""
    pub fn set_input_input_13(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(13, (target.get_id(), 0));
        self
    }

    /// Connects to input 13: "" and specifies the output index of the target node.
    pub fn set_input_input_13_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(13, (target.get_id(), output_index));
        self
    }

    /// Connects to input 14: ""
    pub fn set_input_input_14(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(14, (target.get_id(), 0));
        self
    }

    /// Connects to input 14: "" and specifies the output index of the target node.
    pub fn set_input_input_14_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(14, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_starsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "starsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_starsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "starsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_starrand(mut self, val: f32) -> Self {
        self.base.params.insert(
            "starrand".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_starrand_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "starrand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roundness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roundness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roundness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roundness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_softness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "softness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_softness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "softness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sfreq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sfreq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tfreq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tfreq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "soff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_soff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "toff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_toff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "toff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srand(mut self, val: f32) -> Self {
        self.base.params.insert(
            "srand".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_srand_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trand(mut self, val: f32) -> Self {
        self.base.params.insert(
            "trand".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trand_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blur(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_layers(mut self, val: i32) -> Self {
        self.base.params.insert(
            "layers".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_layers_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "layers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_filter(mut self, val: &str) -> Self {
        self.base.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRstars {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rstars"
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
pub struct VopRust {
    pub base: crate::core::types::NodeBase,
}

impl VopRust {
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

    /// Connects to input 0: ""
    pub fn set_input_input_0(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_rthresh(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rthresh".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rthresh_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rthresh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pthresh(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pthresh".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pthresh_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pthresh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_namp(mut self, val: f32) -> Self {
        self.base.params.insert(
            "namp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_namp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "namp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nrough(mut self, val: f32) -> Self {
        self.base.params.insert(
            "nrough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_nrough_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "nrough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_nfreq(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "nfreq".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_nfreq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "nfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noffset(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "noffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_noffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "noffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "rcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopRust {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "rust"
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
