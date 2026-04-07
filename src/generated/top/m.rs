#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMakedirOutputfiles {
    KeepExistingOutputs = 0,
    AppendDirectoryPathToOutputs = 1,
    ReplaceOutputsWithDirectoryPath = 2,
}

#[derive(Debug, Clone)]
pub struct TopMakedir {
    pub base: crate::core::types::NodeBase,
}

impl TopMakedir {
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

    /// Connects to input 0: "input"
    pub fn set_input_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputfiles(mut self, val: TopMakedirOutputfiles) -> Self {
        self.base.params.insert(
            "outputfiles".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_outputfiles_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputfiles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_dirname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dirname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dirname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dirname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdgnodedep_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("pdgnodedep{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdgnodedep_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("pdgnodedep{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useitemindex(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useitemindex".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useitemindex_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useitemindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopMakedir {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "makedir"
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
pub struct TopMapall {
    pub base: crate::core::types::NodeBase,
}

impl TopMapall {
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

    /// Adds an input automatically to the next available index.
    pub fn add_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base
            .inputs
            .insert(self.base.next_input_index, (target.get_id(), 0));
        self.base.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(self.base.next_input_index, (target.get_id(), output_index));
        self.base.next_input_index += 1;
        self
    }
}

impl crate::core::types::HoudiniNode for TopMapall {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "mapall"
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
pub struct TopMapbyexpression {
    pub base: crate::core::types::NodeBase,
}

impl TopMapbyexpression {
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

    /// Adds an input automatically to the next available index.
    pub fn add_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base
            .inputs
            .insert(self.base.next_input_index, (target.get_id(), 0));
        self.base.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(self.base.next_input_index, (target.get_id(), output_index));
        self.base.next_input_index += 1;
        self
    }

    // --- Int parameters ---
    pub fn with_mapstatic(mut self, val: i32) -> Self {
        self.base.params.insert(
            "mapstatic".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mapstatic_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mapstatic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapdynamic(mut self, val: i32) -> Self {
        self.base.params.insert(
            "mapdynamic".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mapdynamic_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mapdynamic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopMapbyexpression {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "mapbyexpression"
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
pub struct TopMapbyindex {
    pub base: crate::core::types::NodeBase,
}

impl TopMapbyindex {
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

    /// Adds an input automatically to the next available index.
    pub fn add_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base
            .inputs
            .insert(self.base.next_input_index, (target.get_id(), 0));
        self.base.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(self.base.next_input_index, (target.get_id(), output_index));
        self.base.next_input_index += 1;
        self
    }
}

impl crate::core::types::HoudiniNode for TopMapbyindex {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "mapbyindex"
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
pub struct TopMapbyrange {
    pub base: crate::core::types::NodeBase,
}

impl TopMapbyrange {
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

    /// Adds an input automatically to the next available index.
    pub fn add_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base
            .inputs
            .insert(self.base.next_input_index, (target.get_id(), 0));
        self.base.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(self.base.next_input_index, (target.get_id(), output_index));
        self.base.next_input_index += 1;
        self
    }

    // --- Int parameters ---
    pub fn with_upstream_range(mut self, val: i32) -> Self {
        self.base.params.insert(
            "upstream_range".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_upstream_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "upstream_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_downstream_range(mut self, val: i32) -> Self {
        self.base.params.insert(
            "downstream_range".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_downstream_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "downstream_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left_offset(mut self, val: i32) -> Self {
        self.base.params.insert(
            "left_offset".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_left_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left_offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right_offset(mut self, val: i32) -> Self {
        self.base.params.insert(
            "right_offset".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_right_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right_offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopMapbyrange {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "mapbyrange"
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
pub struct TopMatnet {
    pub base: crate::core::types::NodeBase,
}

impl TopMatnet {
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

impl crate::core::types::HoudiniNode for TopMatnet {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "matnet"
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
pub enum TopMayaserverCopyinputs {
    NoIterations = 0,
    FirstIteration = 1,
    AllIterations = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMayaserverPdgCooktype {
    /// Shared Server (Deprecated)
    SharedServerDeprecated = 0,
    Service = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMayaserverPdgServicereset {
    None = 0,
    ResetClient = 1,
    RestartClient = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMayaserverPdgServiceresetwhen {
    BeforeCook = 0,
    AfterCook = 1,
    Both = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMayaserverPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMayaserverPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopMayaserver {
    pub base: crate::core::types::NodeBase,
}

impl TopMayaserver {
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

    /// Connects to input 0: "input"
    pub fn set_input_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_createservice(mut self) -> Self {
        self.base.params.insert(
            "createservice".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_manageservices(mut self) -> Self {
        self.base.params.insert(
            "manageservices".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_commandport(mut self, val: i32) -> Self {
        self.base.params.insert(
            "commandport".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_commandport_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "commandport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeout(mut self, val: i32) -> Self {
        self.base.params.insert(
            "timeout".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_timeout_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copyinputs(mut self, val: TopMayaserverCopyinputs) -> Self {
        self.base.params.insert(
            "copyinputs".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_copyinputs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "copyinputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_cooktype(mut self, val: TopMayaserverPdgCooktype) -> Self {
        self.base.params.insert(
            "pdg_cooktype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_cooktype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_servicereset(mut self, val: TopMayaserverPdgServicereset) -> Self {
        self.base.params.insert(
            "pdg_servicereset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_servicereset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_servicereset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_serviceresetwhen(mut self, val: TopMayaserverPdgServiceresetwhen) -> Self {
        self.base.params.insert(
            "pdg_serviceresetwhen".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_serviceresetwhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_serviceresetwhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopMayaserverPdgWorkitemlabel) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriority(mut self, val: TopMayaserverPdgWorkitempriority) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pdg_feedbackpattern(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_feedbackpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_feedbackpattern_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_feedbackpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalhost(mut self, val: &str) -> Self {
        self.base.params.insert(
            "externalhost".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_externalhost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "externalhost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_serverbinary(mut self, val: &str) -> Self {
        self.base.params.insert(
            "serverbinary".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_serverbinary_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "serverbinary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_servicename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_servicename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_servicename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_servicename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_iterattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "iterattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_iterattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "iterattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sizeattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sizeattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sizeattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sizeattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "numattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_numattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "numattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdgnodedep_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("pdgnodedep{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdgnodedep_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("pdgnodedep{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_iterationsfromupstream(mut self, val: bool) -> Self {
        self.base.params.insert(
            "iterationsfromupstream".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_iterationsfromupstream_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "iterationsfromupstream".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loopsequential(mut self, val: bool) -> Self {
        self.base.params.insert(
            "loopsequential".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loopsequential_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "loopsequential".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_feedbackattribs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_feedbackattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_feedbackattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_feedbackattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_feedbackfiles(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_feedbackfiles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_feedbackfiles_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_feedbackfiles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalserver(mut self, val: bool) -> Self {
        self.base.params.insert(
            "externalserver".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_externalserver_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "externalserver".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopMayaserver {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "mayaserver"
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
pub struct TopMerge {
    pub base: crate::core::types::NodeBase,
}

impl TopMerge {
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

    /// Adds an input automatically to the next available index.
    pub fn add_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base
            .inputs
            .insert(self.base.next_input_index, (target.get_id(), 0));
        self.base.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(self.base.next_input_index, (target.get_id(), output_index));
        self.base.next_input_index += 1;
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_extrainput_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("extrainput{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_extrainput_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("extrainput{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_keepfailed(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keepfailed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepfailed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keepfailed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableextrainput_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("enableextrainput{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableextrainput_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("enableextrainput{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopMerge {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "merge"
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
pub enum TopMlPreprocessoidnCleanauxiliary {
    No = 0,
    Yes = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlPreprocessoidnDeterministic {
    Yes = 0,
    No = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlPreprocessoidnPythonbin {
    Hython = 0,
    PdgPython = 1,
    Custom = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlPreprocessoidnPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    UseCustomExpression = 2,
    NodeDefinesLabel = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlPreprocessoidnPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    UseCustomExpression = 1,
    NodeDefinesPriority = 2,
}

#[derive(Debug, Clone)]
pub struct TopMlPreprocessoidn {
    pub base: crate::core::types::NodeBase,
}

impl TopMlPreprocessoidn {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_deviceid(mut self, val: i32) -> Self {
        self.base.params.insert(
            "deviceid".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deviceid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deviceid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_devicecount(mut self, val: i32) -> Self {
        self.base.params.insert(
            "devicecount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_devicecount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "devicecount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_cleanauxiliary(mut self, val: TopMlPreprocessoidnCleanauxiliary) -> Self {
        self.base.params.insert(
            "cleanauxiliary".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cleanauxiliary_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cleanauxiliary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deterministic(mut self, val: TopMlPreprocessoidnDeterministic) -> Self {
        self.base.params.insert(
            "deterministic".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_deterministic_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deterministic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pythonbin(mut self, val: TopMlPreprocessoidnPythonbin) -> Self {
        self.base.params.insert(
            "pythonbin".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pythonbin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pythonbin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopMlPreprocessoidnPdgWorkitemlabel) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriority(
        mut self,
        val: TopMlPreprocessoidnPdgWorkitempriority,
    ) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_inputdirectory(mut self, val: &str) -> Self {
        self.base.params.insert(
            "inputdirectory".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputdirectory_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputdirectory".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trainingdataset(mut self, val: &str) -> Self {
        self.base.params.insert(
            "trainingdataset".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_trainingdataset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trainingdataset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_validationdataset(mut self, val: &str) -> Self {
        self.base.params.insert(
            "validationdataset".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_validationdataset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "validationdataset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extension(mut self, val: &str) -> Self {
        self.base.params.insert(
            "extension".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_extension_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extension".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputdirectory(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputdirectory".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputdirectory_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputdirectory".to_string(),
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
    pub fn with_inputfeatures(mut self, val: &str) -> Self {
        self.base.params.insert(
            "inputfeatures".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputfeatures_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputfeatures".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transfer(mut self, val: &str) -> Self {
        self.base.params.insert(
            "transfer".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_transfer_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "transfer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_devicename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "devicename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_devicename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "devicename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_venvpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "venvpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_venvpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "venvpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_python(mut self, val: &str) -> Self {
        self.base.params.insert(
            "python".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_python_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "python".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usetransfer(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usetransfer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetransfer_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usetransfer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedevicename(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usedevicename".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedevicename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usedevicename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_venvsymlink(mut self, val: bool) -> Self {
        self.base.params.insert(
            "venvsymlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_venvsymlink_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "venvsymlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepipcache(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usepipcache".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepipcache_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usepipcache".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopMlPreprocessoidn {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ml_preprocessoidn"
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
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait TopMlPreprocessoidnInnerExt {
    fn load_parameters(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn partitionbyindex1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pythonscript1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pythonvenv1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> TopMlPreprocessoidnInnerExt for crate::core::graph::InnerGraph<'a, TopMlPreprocessoidn> {
    fn load_parameters(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("load_parameters")
    }
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("output0")
    }
    fn partitionbyindex1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("partitionbyindex1")
    }
    fn pythonscript1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("pythonscript1")
    }
    fn pythonvenv1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("pythonvenv1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlRegressionkernelKerneltype {
    Gaussian = 0,
    Polynomial = 1,
    Sigmoid = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlRegressionkernelPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    UseCustomExpression = 2,
    NodeDefinesLabel = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlRegressionkernelPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    UseCustomExpression = 1,
    NodeDefinesPriority = 2,
}

#[derive(Debug, Clone)]
pub struct TopMlRegressionkernel {
    pub base: crate::core::types::NodeBase,
}

impl TopMlRegressionkernel {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_weightdecay(mut self, val: f32) -> Self {
        self.base.params.insert(
            "weightdecay".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weightdecay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "weightdecay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_errorthreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "errorthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_errorthreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "errorthreshold".to_string(),
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
    pub fn with_polynomialoffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "polynomialoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_polynomialoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "polynomialoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sigmoidscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sigmoidscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sigmoidscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sigmoidscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sigmoidoffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sigmoidoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sigmoidoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sigmoidoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_polynomialdegree(mut self, val: i32) -> Self {
        self.base.params.insert(
            "polynomialdegree".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_polynomialdegree_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "polynomialdegree".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_kerneltype(mut self, val: TopMlRegressionkernelKerneltype) -> Self {
        self.base.params.insert(
            "kerneltype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_kerneltype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "kerneltype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_cachemode(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_cachemode".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_cachemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_cachemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopMlRegressionkernelPdgWorkitemlabel) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriority(
        mut self,
        val: TopMlRegressionkernelPdgWorkitempriority,
    ) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_datasetfolder(mut self, val: &str) -> Self {
        self.base.params.insert(
            "datasetfolder".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_datasetfolder_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "datasetfolder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_datasetbasename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "datasetbasename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_datasetbasename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "datasetbasename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kernelfolder(mut self, val: &str) -> Self {
        self.base.params.insert(
            "kernelfolder".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kernelfolder_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "kernelfolder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kernelbasename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "kernelbasename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kernelbasename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "kernelbasename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modelsfolder(mut self, val: &str) -> Self {
        self.base.params.insert(
            "modelsfolder".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_modelsfolder_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "modelsfolder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modelbasename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "modelbasename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_modelbasename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "modelbasename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_logsfolder(mut self, val: &str) -> Self {
        self.base.params.insert(
            "logsfolder".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_logsfolder_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "logsfolder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_logbasename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "logbasename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_logbasename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "logbasename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_venvpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "venvpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_venvpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "venvpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_logtostandardoutput(mut self, val: bool) -> Self {
        self.base.params.insert(
            "logtostandardoutput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_logtostandardoutput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "logtostandardoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepipcache(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usepipcache".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepipcache_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usepipcache".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopMlRegressionkernel {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ml_regressionkernel"
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
pub enum TopMlTrainoidnCleanauxiliary {
    No = 0,
    Yes = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainoidnDeterministic {
    Yes = 0,
    No = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainoidnPythonbin {
    Hython = 0,
    PdgPython = 1,
    Custom = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainoidnPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainoidnPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopMlTrainoidn {
    pub base: crate::core::types::NodeBase,
}

impl TopMlTrainoidn {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_learningrate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "learningrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_learningrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "learningrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxlearningrate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxlearningrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxlearningrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxlearningrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_learningratewarmup(mut self, val: f32) -> Self {
        self.base.params.insert(
            "learningratewarmup".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_learningratewarmup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "learningratewarmup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- FloatArray parameters ---
    pub fn with_msssimweights(mut self, val: Vec<f32>) -> Self {
        self.base.params.insert(
            "msssimweights".to_string(),
            crate::core::types::ParamValue::FloatArray(val),
        );
        self
    }
    pub fn with_msssimweights_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "msssimweights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_epochs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "epochs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_epochs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "epochs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_validationrate(mut self, val: i32) -> Self {
        self.base.params.insert(
            "validationrate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_validationrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "validationrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointrate(mut self, val: i32) -> Self {
        self.base.params.insert(
            "checkpointrate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_checkpointrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "checkpointrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tilesize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "tilesize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_tilesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tilesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_batchsize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "batchsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_batchsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "batchsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_workerthreads(mut self, val: i32) -> Self {
        self.base.params.insert(
            "workerthreads".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_workerthreads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "workerthreads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed(mut self, val: i32) -> Self {
        self.base
            .params
            .insert("seed".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportepoch(mut self, val: i32) -> Self {
        self.base.params.insert(
            "exportepoch".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_exportepoch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exportepoch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deviceid(mut self, val: i32) -> Self {
        self.base.params.insert(
            "deviceid".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deviceid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deviceid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_devicecount(mut self, val: i32) -> Self {
        self.base.params.insert(
            "devicecount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_devicecount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "devicecount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_cleanauxiliary(mut self, val: TopMlTrainoidnCleanauxiliary) -> Self {
        self.base.params.insert(
            "cleanauxiliary".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cleanauxiliary_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cleanauxiliary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deterministic(mut self, val: TopMlTrainoidnDeterministic) -> Self {
        self.base.params.insert(
            "deterministic".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_deterministic_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deterministic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pythonbin(mut self, val: TopMlTrainoidnPythonbin) -> Self {
        self.base.params.insert(
            "pythonbin".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pythonbin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pythonbin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopMlTrainoidnPdgWorkitemlabel) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriority(mut self, val: TopMlTrainoidnPdgWorkitempriority) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_inputdirectory(mut self, val: &str) -> Self {
        self.base.params.insert(
            "inputdirectory".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputdirectory_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputdirectory".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trainingdataset(mut self, val: &str) -> Self {
        self.base.params.insert(
            "trainingdataset".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_trainingdataset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trainingdataset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_validationdataset(mut self, val: &str) -> Self {
        self.base.params.insert(
            "validationdataset".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_validationdataset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "validationdataset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputdirectory(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputdirectory".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputdirectory_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputdirectory".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loss(mut self, val: &str) -> Self {
        self.base.params.insert(
            "loss".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_loss_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "loss".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "precision".to_string(),
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
    pub fn with_inputfeatures(mut self, val: &str) -> Self {
        self.base.params.insert(
            "inputfeatures".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputfeatures_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputfeatures".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transfer(mut self, val: &str) -> Self {
        self.base.params.insert(
            "transfer".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_transfer_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "transfer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportdirectory(mut self, val: &str) -> Self {
        self.base.params.insert(
            "exportdirectory".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_exportdirectory_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exportdirectory".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportfile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "exportfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_exportfile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exportfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_devicename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "devicename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_devicename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "devicename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_venvpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "venvpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_venvpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "venvpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_python(mut self, val: &str) -> Self {
        self.base.params.insert(
            "python".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_python_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "python".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uselearningrate(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uselearningrate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uselearningrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uselearningrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemaxlearningrate(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usemaxlearningrate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaxlearningrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usemaxlearningrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useseed(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useseed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useseed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verbose(mut self, val: bool) -> Self {
        self.base.params.insert(
            "verbose".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_verbose_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "verbose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetransfer(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usetransfer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetransfer_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usetransfer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableexport(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableexport".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableexport_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableexport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useexportepoch(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useexportepoch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useexportepoch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useexportepoch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedevicename(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usedevicename".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedevicename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usedevicename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_venvsymlink(mut self, val: bool) -> Self {
        self.base.params.insert(
            "venvsymlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_venvsymlink_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "venvsymlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepipcache(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usepipcache".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepipcache_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usepipcache".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopMlTrainoidn {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ml_trainoidn"
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
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait TopMlTrainoidnInnerExt {
    fn load_parameters(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn partitionbyindex1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pythonscript1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pythonvenv1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> TopMlTrainoidnInnerExt for crate::core::graph::InnerGraph<'a, TopMlTrainoidn> {
    fn load_parameters(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("load_parameters")
    }
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("output0")
    }
    fn partitionbyindex1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("partitionbyindex1")
    }
    fn pythonscript1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("pythonscript1")
    }
    fn pythonvenv1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("pythonvenv1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainregressionUniformhiddenactivation {
    Tanh = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainregressionPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    UseCustomExpression = 2,
    NodeDefinesLabel = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainregressionPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    UseCustomExpression = 1,
    NodeDefinesPriority = 2,
}

#[derive(Debug, Clone)]
pub struct TopMlTrainregression {
    pub base: crate::core::types::NodeBase,
}

impl TopMlTrainregression {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_weightdecay(mut self, val: f32) -> Self {
        self.base.params.insert(
            "weightdecay".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weightdecay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "weightdecay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_learningrate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "learningrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_learningrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "learningrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_betaone(mut self, val: f32) -> Self {
        self.base.params.insert(
            "betaone".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_betaone_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "betaone".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_betatwo(mut self, val: f32) -> Self {
        self.base.params.insert(
            "betatwo".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_betatwo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "betatwo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adarho(mut self, val: f32) -> Self {
        self.base.params.insert(
            "adarho".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_adarho_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adarho".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sgdmomentum(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sgdmomentum".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sgdmomentum_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sgdmomentum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stepgamma(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stepgamma".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stepgamma_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stepgamma".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exponentialgamma(mut self, val: f32) -> Self {
        self.base.params.insert(
            "exponentialgamma".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_exponentialgamma_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exponentialgamma".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_endlearningrate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "endlearningrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_endlearningrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "endlearningrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_uniformhiddenlayers(mut self, val: i32) -> Self {
        self.base.params.insert(
            "uniformhiddenlayers".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uniformhiddenlayers_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniformhiddenlayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformhiddenwidth(mut self, val: i32) -> Self {
        self.base.params.insert(
            "uniformhiddenwidth".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uniformhiddenwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniformhiddenwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weightinitializationrandomseed(mut self, val: i32) -> Self {
        self.base.params.insert(
            "weightinitializationrandomseed".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_weightinitializationrandomseed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "weightinitializationrandomseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shufflerandomseed(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shufflerandomseed".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shufflerandomseed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shufflerandomseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upperlimit(mut self, val: i32) -> Self {
        self.base.params.insert(
            "upperlimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_upperlimit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "upperlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxepochs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxepochs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxepochs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxepochs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trainingdataproportion(mut self, val: i32) -> Self {
        self.base.params.insert(
            "trainingdataproportion".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_trainingdataproportion_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trainingdataproportion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_validationdataproportion(mut self, val: i32) -> Self {
        self.base.params.insert(
            "validationdataproportion".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_validationdataproportion_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "validationdataproportion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_epochsperevaluation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "epochsperevaluation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_epochsperevaluation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "epochsperevaluation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxiterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxiterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_patience(mut self, val: i32) -> Self {
        self.base.params.insert(
            "patience".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_patience_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "patience".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lineardecay(mut self, val: i32) -> Self {
        self.base.params.insert(
            "lineardecay".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lineardecay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lineardecay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stepsize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "stepsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stepsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stepsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxbatchsize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxbatchsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxbatchsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxbatchsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modelsaveepochsperoutput(mut self, val: i32) -> Self {
        self.base.params.insert(
            "modelsaveepochsperoutput".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_modelsaveepochsperoutput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "modelsaveepochsperoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modelexportepochsperoutput(mut self, val: i32) -> Self {
        self.base.params.insert(
            "modelexportepochsperoutput".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_modelexportepochsperoutput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "modelexportepochsperoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_networkcomposition(mut self, val: i32) -> Self {
        self.base.params.insert(
            "networkcomposition".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_networkcomposition_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "networkcomposition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modelkind(mut self, val: i32) -> Self {
        self.base.params.insert(
            "modelkind".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_modelkind_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "modelkind".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modelstandardtype(mut self, val: i32) -> Self {
        self.base.params.insert(
            "modelstandardtype".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_modelstandardtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "modelstandardtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hiddenlayerformat(mut self, val: i32) -> Self {
        self.base.params.insert(
            "hiddenlayerformat".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_hiddenlayerformat_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hiddenlayerformat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformhiddenactivation(
        mut self,
        val: TopMlTrainregressionUniformhiddenactivation,
    ) -> Self {
        self.base.params.insert(
            "uniformhiddenactivation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uniformhiddenactivation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniformhiddenactivation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_losskind(mut self, val: i32) -> Self {
        self.base.params.insert(
            "losskind".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_losskind_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "losskind".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lossstandardtype(mut self, val: i32) -> Self {
        self.base.params.insert(
            "lossstandardtype".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_lossstandardtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lossstandardtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optimizertype(mut self, val: i32) -> Self {
        self.base.params.insert(
            "optimizertype".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_optimizertype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "optimizertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_schedulertype(mut self, val: i32) -> Self {
        self.base.params.insert(
            "schedulertype".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_schedulertype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "schedulertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exponentialstyle(mut self, val: i32) -> Self {
        self.base.params.insert(
            "exponentialstyle".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_exponentialstyle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exponentialstyle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modelsaveevent(mut self, val: i32) -> Self {
        self.base.params.insert(
            "modelsaveevent".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_modelsaveevent_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "modelsaveevent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modelexportevent(mut self, val: i32) -> Self {
        self.base.params.insert(
            "modelexportevent".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_modelexportevent_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "modelexportevent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_losslogevent(mut self, val: i32) -> Self {
        self.base.params.insert(
            "losslogevent".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_losslogevent_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "losslogevent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_cachemode(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_cachemode".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_cachemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_cachemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopMlTrainregressionPdgWorkitemlabel) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriority(
        mut self,
        val: TopMlTrainregressionPdgWorkitempriority,
    ) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_modelcreationscript(mut self, val: &str) -> Self {
        self.base.params.insert(
            "modelcreationscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_modelcreationscript_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "modelcreationscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_losscreationscript(mut self, val: &str) -> Self {
        self.base.params.insert(
            "losscreationscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_losscreationscript_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "losscreationscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unifiedcreationscript(mut self, val: &str) -> Self {
        self.base.params.insert(
            "unifiedcreationscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_unifiedcreationscript_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unifiedcreationscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hyperparameterfolder(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hyperparameterfolder".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hyperparameterfolder_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hyperparameterfolder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hyperparameterbasename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hyperparameterbasename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hyperparameterbasename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hyperparameterbasename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_datasetfolder(mut self, val: &str) -> Self {
        self.base.params.insert(
            "datasetfolder".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_datasetfolder_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "datasetfolder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_datasetbasename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "datasetbasename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_datasetbasename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "datasetbasename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modelsfolder(mut self, val: &str) -> Self {
        self.base.params.insert(
            "modelsfolder".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_modelsfolder_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "modelsfolder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modelbasename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "modelbasename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_modelbasename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "modelbasename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_logsfolder(mut self, val: &str) -> Self {
        self.base.params.insert(
            "logsfolder".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_logsfolder_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "logsfolder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_logbasename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "logbasename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_logbasename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "logbasename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_statesfolder(mut self, val: &str) -> Self {
        self.base.params.insert(
            "statesfolder".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_statesfolder_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "statesfolder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_statebasename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "statebasename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_statebasename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "statebasename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_venvpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "venvpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_venvpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "venvpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_shuffle(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shuffle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shuffle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shuffle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limitsize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "limitsize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "limitsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limitepochs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "limitepochs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitepochs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "limitepochs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablevalidation(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablevalidation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablevalidation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablevalidation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableearlystopping(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableearlystopping".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableearlystopping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableearlystopping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modelsaveappendepoch(mut self, val: bool) -> Self {
        self.base.params.insert(
            "modelsaveappendepoch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_modelsaveappendepoch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "modelsaveappendepoch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modelexportappendepoch(mut self, val: bool) -> Self {
        self.base.params.insert(
            "modelexportappendepoch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_modelexportappendepoch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "modelexportappendepoch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_logtostandardoutput(mut self, val: bool) -> Self {
        self.base.params.insert(
            "logtostandardoutput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_logtostandardoutput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "logtostandardoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecpuexclusively(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usecpuexclusively".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecpuexclusively_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usecpuexclusively".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepipcache(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usepipcache".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepipcache_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usepipcache".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopMlTrainregression {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ml_trainregression"
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
pub enum TopMlTrainstyletransferInputtype {
    SinglePairedImage = 0,
    MultiplePairedImages = 1,
    MultipleUnpairedImages = 2,
    CompositePairedImages = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainstyletransferInputstorage {
    Automatic = 0,
    KeepOnTrainingDevice = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainstyletransferTesttype {
    SinglePairedImage = 0,
    MultiplePairedImages = 1,
    MultipleUnpairedImages = 2,
    CompositePairedImages = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainstyletransferTeststorage {
    Automatic = 0,
    KeepOnTrainingDevice = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainstyletransferPythonbin {
    Hython = 0,
    PdgPython = 1,
    Custom = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainstyletransferPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopMlTrainstyletransferPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopMlTrainstyletransfer {
    pub base: crate::core::types::NodeBase,
}

impl TopMlTrainstyletransfer {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_randomcropping(mut self, val: f32) -> Self {
        self.base.params.insert(
            "randomcropping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomcropping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "randomcropping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_horizontalflip(mut self, val: f32) -> Self {
        self.base.params.insert(
            "horizontalflip".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_horizontalflip_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "horizontalflip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verticalflip(mut self, val: f32) -> Self {
        self.base.params.insert(
            "verticalflip".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_verticalflip_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "verticalflip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotation(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rotation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rotation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_learningrate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "learningrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_learningrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "learningrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stepgamma(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stepgamma".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stepgamma_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stepgamma".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initializergain(mut self, val: f32) -> Self {
        self.base.params.insert(
            "initializergain".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_initializergain_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "initializergain".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adarho(mut self, val: f32) -> Self {
        self.base.params.insert(
            "adarho".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_adarho_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adarho".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sgdmomentum(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sgdmomentum".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sgdmomentum_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sgdmomentum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_generatorlambda(mut self, val: f32) -> Self {
        self.base.params.insert(
            "generatorlambda".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_generatorlambda_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "generatorlambda".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_testssim(mut self, val: f32) -> Self {
        self.base.params.insert(
            "testssim".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_testssim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "testssim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_adambeta(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "adambeta".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_adambeta_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adambeta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_imagechannels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "imagechannels".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_imagechannels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "imagechannels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imagesize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "imagesize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_imagesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "imagesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputthreads(mut self, val: i32) -> Self {
        self.base.params.insert(
            "inputthreads".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_inputthreads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputthreads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputbatchsize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "inputbatchsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_inputbatchsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputbatchsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maximages(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maximages".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maximages_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maximages".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxiterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxiterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lineardecay(mut self, val: i32) -> Self {
        self.base.params.insert(
            "lineardecay".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_lineardecay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lineardecay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stepsize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "stepsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stepsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stepsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_generatorlayers(mut self, val: i32) -> Self {
        self.base.params.insert(
            "generatorlayers".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_generatorlayers_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "generatorlayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_generatorinitial(mut self, val: i32) -> Self {
        self.base.params.insert(
            "generatorinitial".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_generatorinitial_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "generatorinitial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_generatorkernel(mut self, val: i32) -> Self {
        self.base.params.insert(
            "generatorkernel".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_generatorkernel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "generatorkernel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_discriminatorlayers(mut self, val: i32) -> Self {
        self.base.params.insert(
            "discriminatorlayers".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_discriminatorlayers_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "discriminatorlayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_discriminatorinitial(mut self, val: i32) -> Self {
        self.base.params.insert(
            "discriminatorinitial".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_discriminatorinitial_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "discriminatorinitial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_discriminatorkernel(mut self, val: i32) -> Self {
        self.base.params.insert(
            "discriminatorkernel".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_discriminatorkernel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "discriminatorkernel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_testthreads(mut self, val: i32) -> Self {
        self.base.params.insert(
            "testthreads".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_testthreads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "testthreads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_testbatchsize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "testbatchsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_testbatchsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "testbatchsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_testrate(mut self, val: i32) -> Self {
        self.base.params.insert(
            "testrate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_testrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "testrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_testcount(mut self, val: i32) -> Self {
        self.base.params.insert(
            "testcount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_testcount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "testcount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modelrate(mut self, val: i32) -> Self {
        self.base.params.insert(
            "modelrate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_modelrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "modelrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onnxrate(mut self, val: i32) -> Self {
        self.base.params.insert(
            "onnxrate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_onnxrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "onnxrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onnxversion(mut self, val: i32) -> Self {
        self.base.params.insert(
            "onnxversion".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_onnxversion_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "onnxversion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_plotrate(mut self, val: i32) -> Self {
        self.base.params.insert(
            "plotrate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_plotrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "plotrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cputhreads(mut self, val: i32) -> Self {
        self.base.params.insert(
            "cputhreads".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cputhreads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cputhreads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_inputtype(mut self, val: TopMlTrainstyletransferInputtype) -> Self {
        self.base.params.insert(
            "inputtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inputtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputstorage(mut self, val: TopMlTrainstyletransferInputstorage) -> Self {
        self.base.params.insert(
            "inputstorage".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inputstorage_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputstorage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_schedulertype(mut self, val: i32) -> Self {
        self.base.params.insert(
            "schedulertype".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_schedulertype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "schedulertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initializertype(mut self, val: i32) -> Self {
        self.base.params.insert(
            "initializertype".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_initializertype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "initializertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optimizertype(mut self, val: i32) -> Self {
        self.base.params.insert(
            "optimizertype".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_optimizertype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "optimizertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_generatordownactivation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "generatordownactivation".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_generatordownactivation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "generatordownactivation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_generatorupactivation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "generatorupactivation".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_generatorupactivation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "generatorupactivation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_discriminatoractivation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "discriminatoractivation".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_discriminatoractivation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "discriminatoractivation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_testtype(mut self, val: TopMlTrainstyletransferTesttype) -> Self {
        self.base.params.insert(
            "testtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_testtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "testtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_teststorage(mut self, val: TopMlTrainstyletransferTeststorage) -> Self {
        self.base.params.insert(
            "teststorage".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_teststorage_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "teststorage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_plottype(mut self, val: i32) -> Self {
        self.base.params.insert(
            "plottype".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_plottype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "plottype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pythonbin(mut self, val: TopMlTrainstyletransferPythonbin) -> Self {
        self.base.params.insert(
            "pythonbin".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pythonbin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pythonbin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopMlTrainstyletransferPdgWorkitemlabel) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriority(
        mut self,
        val: TopMlTrainstyletransferPdgWorkitempriority,
    ) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_singleinput(mut self, val: &str) -> Self {
        self.base.params.insert(
            "singleinput".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_singleinput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "singleinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_singlereference(mut self, val: &str) -> Self {
        self.base.params.insert(
            "singlereference".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_singlereference_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "singlereference".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputdir(mut self, val: &str) -> Self {
        self.base.params.insert(
            "inputdir".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputdir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_referencedir(mut self, val: &str) -> Self {
        self.base.params.insert(
            "referencedir".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_referencedir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "referencedir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compositedir(mut self, val: &str) -> Self {
        self.base.params.insert(
            "compositedir".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_compositedir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "compositedir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_testoutput(mut self, val: &str) -> Self {
        self.base.params.insert(
            "testoutput".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_testoutput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "testoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_singletestinput(mut self, val: &str) -> Self {
        self.base.params.insert(
            "singletestinput".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_singletestinput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "singletestinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_singletestreference(mut self, val: &str) -> Self {
        self.base.params.insert(
            "singletestreference".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_singletestreference_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "singletestreference".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_testinputdir(mut self, val: &str) -> Self {
        self.base.params.insert(
            "testinputdir".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_testinputdir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "testinputdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_testreferencedir(mut self, val: &str) -> Self {
        self.base.params.insert(
            "testreferencedir".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_testreferencedir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "testreferencedir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_testcompositedir(mut self, val: &str) -> Self {
        self.base.params.insert(
            "testcompositedir".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_testcompositedir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "testcompositedir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modelformat(mut self, val: &str) -> Self {
        self.base.params.insert(
            "modelformat".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_modelformat_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "modelformat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onnxformat(mut self, val: &str) -> Self {
        self.base.params.insert(
            "onnxformat".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_onnxformat_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "onnxformat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ssimplot(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ssimplot".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ssimplot_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ssimplot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lossplot(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lossplot".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lossplot_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lossplot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scoreplot(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scoreplot".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scoreplot_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scoreplot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_devicename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "devicename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_devicename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "devicename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_venvpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "venvpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_venvpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "venvpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_python(mut self, val: &str) -> Self {
        self.base.params.insert(
            "python".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_python_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "python".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usemaximages(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usemaximages".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaximages_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usemaximages".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_userandomcropping(mut self, val: bool) -> Self {
        self.base.params.insert(
            "userandomcropping".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userandomcropping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "userandomcropping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usehorizontalflip(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usehorizontalflip".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usehorizontalflip_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usehorizontalflip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useverticalflip(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useverticalflip".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useverticalflip_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useverticalflip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_userotation(mut self, val: bool) -> Self {
        self.base.params.insert(
            "userotation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userotation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "userotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imageshuffle(mut self, val: bool) -> Self {
        self.base.params.insert(
            "imageshuffle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_imageshuffle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "imageshuffle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_testmodel(mut self, val: bool) -> Self {
        self.base.params.insert(
            "testmodel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_testmodel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "testmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetestssim(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usetestssim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetestssim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usetestssim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addtestoutputs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addtestoutputs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addtestoutputs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addtestoutputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_writemodels(mut self, val: bool) -> Self {
        self.base.params.insert(
            "writemodels".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_writemodels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "writemodels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addmodeloutput(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addmodeloutput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addmodeloutput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addmodeloutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_writeonnx(mut self, val: bool) -> Self {
        self.base.params.insert(
            "writeonnx".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_writeonnx_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "writeonnx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onnxdynamic(mut self, val: bool) -> Self {
        self.base.params.insert(
            "onnxdynamic".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_onnxdynamic_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "onnxdynamic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addonnxoutputs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addonnxoutputs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addonnxoutputs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addonnxoutputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addattributes(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addattributes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedevicename(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usedevicename".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedevicename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usedevicename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecputhreads(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usecputhreads".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecputhreads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usecputhreads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_venvsymlink(mut self, val: bool) -> Self {
        self.base.params.insert(
            "venvsymlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_venvsymlink_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "venvsymlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepipcache(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usepipcache".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepipcache_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usepipcache".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopMlTrainstyletransfer {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ml_trainstyletransfer"
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
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait TopMlTrainstyletransferInnerExt {
    fn load_parameters(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn partitionbyindex1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pythonscript1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn pythonvenv1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> TopMlTrainstyletransferInnerExt
    for crate::core::graph::InnerGraph<'a, TopMlTrainstyletransfer>
{
    fn load_parameters(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("load_parameters")
    }
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("output0")
    }
    fn partitionbyindex1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("partitionbyindex1")
    }
    fn pythonscript1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("pythonscript1")
    }
    fn pythonvenv1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("pythonvenv1")
    }
}
