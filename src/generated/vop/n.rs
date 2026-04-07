#[derive(Debug, Clone)]
pub struct VopNearpoint {
    pub base: crate::core::types::NodeBase,
}

impl VopNearpoint {
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
    pub fn with_input(mut self, val: &str) -> Self {
        self.base.params.insert(
            "input".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_input_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "input".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopNearpoint {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "nearpoint"
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
pub struct VopNegate {
    pub base: crate::core::types::NodeBase,
}

impl VopNegate {
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

impl crate::core::types::HoudiniNode for VopNegate {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "negate"
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
pub struct VopNeighbour {
    pub base: crate::core::types::NodeBase,
}

impl VopNeighbour {
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
    pub fn with_neighnum(mut self, val: i32) -> Self {
        self.base.params.insert(
            "neighnum".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_neighnum_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "neighnum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopNeighbour {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "neighbour"
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
pub struct VopNeighbourcount {
    pub base: crate::core::types::NodeBase,
}

impl VopNeighbourcount {
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
}

impl crate::core::types::HoudiniNode for VopNeighbourcount {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "neighbourcount"
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
pub struct VopNeighbourcountfile {
    pub base: crate::core::types::NodeBase,
}

impl VopNeighbourcountfile {
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
    pub fn with_filename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "filename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopNeighbourcountfile {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "neighbourcountfile"
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
pub struct VopNeighbourfile {
    pub base: crate::core::types::NodeBase,
}

impl VopNeighbourfile {
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
    pub fn with_neighnum(mut self, val: i32) -> Self {
        self.base.params.insert(
            "neighnum".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_neighnum_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "neighnum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
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
    pub fn with_filename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "filename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopNeighbourfile {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "neighbourfile"
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
pub struct VopNeighbours {
    pub base: crate::core::types::NodeBase,
}

impl VopNeighbours {
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
    pub fn with_filename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "filename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopNeighbours {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "neighbours"
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
pub struct VopNewgroup {
    pub base: crate::core::types::NodeBase,
}

impl VopNewgroup {
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

impl crate::core::types::HoudiniNode for VopNewgroup {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "newgroup"
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
pub struct VopNormalclamp {
    pub base: crate::core::types::NodeBase,
}

impl VopNormalclamp {
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
    pub fn with_nn(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "nN".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_nn_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "nN".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nng(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "nNg".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_nng_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "nNg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ni(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "nI".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ni_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "nI".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopNormalclamp {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "normalclamp"
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
pub struct VopNormalfalloff {
    pub base: crate::core::types::NodeBase,
}

impl VopNormalfalloff {
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
    pub fn with_exp(mut self, val: f32) -> Self {
        self.base.params.insert(
            "exp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_exp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_invert(mut self, val: bool) -> Self {
        self.base.params.insert(
            "invert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_invert_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "invert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

impl crate::core::types::HoudiniNode for VopNormalfalloff {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "normalfalloff"
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
pub struct VopNormalize {
    pub base: crate::core::types::NodeBase,
}

impl VopNormalize {
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

    // --- Float2 parameters ---
    pub fn with_vec_u(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "vec_u".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_vec_u_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vec_u".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_vec(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vec".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vec_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vec_p(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vec_p".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vec_p_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vec_p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vec_v(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vec_v".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vec_v_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vec_v".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vec_un(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vec_un".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vec_un_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vec_un".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vec_up(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vec_up".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vec_up_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vec_up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vec_uv(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vec_uv".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vec_uv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vec_uv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_vec_v4(mut self, val: [f32; 4]) -> Self {
        self.base.params.insert(
            "vec_v4".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_vec_v4_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vec_v4".to_string(),
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

impl crate::core::types::HoudiniNode for VopNormalize {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "normalize"
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
pub struct VopNot {
    pub base: crate::core::types::NodeBase,
}

impl VopNot {
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

    // --- Toggle parameters ---
    pub fn with_bitwise(mut self, val: bool) -> Self {
        self.base.params.insert(
            "bitwise".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bitwise_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bitwise".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopNot {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "not"
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
pub struct VopNpoints {
    pub base: crate::core::types::NodeBase,
}

impl VopNpoints {
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
    pub fn with_input_opi(mut self, val: i32) -> Self {
        self.base.params.insert(
            "input_opi".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_input_opi_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "input_opi".to_string(),
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
    pub fn with_input(mut self, val: &str) -> Self {
        self.base.params.insert(
            "input".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_input_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "input".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopNpoints {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "npoints"
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
pub struct VopNrandom {
    pub base: crate::core::types::NodeBase,
}

impl VopNrandom {
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
    pub fn with_scheme(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scheme".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scheme_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scheme".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopNrandom {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "nrandom"
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
pub struct VopNull {
    pub base: crate::core::types::NodeBase,
}

impl VopNull {
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

    // --- String parameters ---
    pub fn with_outputname_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("outputname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("outputname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopNull {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "null"
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
pub struct VopNuniqueval {
    pub base: crate::core::types::NodeBase,
}

impl VopNuniqueval {
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
    pub fn with_filename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "filename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_atype(mut self, val: &str) -> Self {
        self.base.params.insert(
            "atype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_atype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "atype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "attrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for VopNuniqueval {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "nuniqueval"
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
