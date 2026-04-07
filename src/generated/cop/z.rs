#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopZcompConvertdepth {
    Distance = 0,
    Depth = 1,
    Height = 2,
}

#[derive(Debug, Clone)]
pub struct CopZcomp {
    pub base: crate::core::types::NodeBase,
}

impl CopZcomp {
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
    pub fn with_fgoffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fgoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fgoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fgoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_convertdepth(mut self, val: CopZcompConvertdepth) -> Self {
        self.base.params.insert(
            "convertdepth".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_convertdepth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "convertdepth".to_string(),
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

impl crate::core::types::HoudiniNode for CopZcomp {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "zcomp"
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
